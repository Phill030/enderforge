use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// # Information
/// Makes the struct serializable for T & Vec<T>
#[proc_macro_derive(Serializable, attributes(serialize_as))]
pub fn derive_serializable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let attributes = &ast.attrs;

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);
                let field_names_2 = field_names.clone();
                quote! {
                    impl Encoder for #struct_name {
                        async fn encode<W: tokio::io::AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
                            #(self.#field_names.encode(writer).await?;)*
                            Ok(())
                        }
                    }

                    impl Encoder for Vec<#struct_name> {
                        async fn encode<W: tokio::io::AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), EncodeError> {
                            writer.write_var_i32(VarInt::from(self.len())).await?;

                            for x in self.to_vec() {
                                #(x.#field_names_2.encode(writer).await?;)*
                            }

                            Ok(())
                        }
                    }
                }
            }
            .into(),
            _ => panic!("Expected a named field"),
        },
        // Data::Enum(data_enum) => {
        // let attribute = attributes
        //     .iter()
        //     .filter(|a| a.path().is_ident("serialize_as"))
        //     .nth(0)
        //     .expect("Expected a Type (#[serialize_as(i32)]");

        // let serialize_type: syn::Type = {
        //     let ty: syn::Type = attribute.parse_args().expect("Expected a Type");
        //     ty
        // };

        // let variants = data_enum.variants.iter().map(|variant| &variant.ident);
        // let gen = quote! {};

        // gen.into()
        // }
        _ => panic!("Expected a struct or an enum with named fields"),
    }
}

#[proc_macro_derive(Streamable, attributes(packet_id))]
pub fn derive_streamable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let attributes = &ast.attrs;

    let attribute = attributes
        .iter()
        .filter(|a| a.path().is_ident("packet_id"))
        .nth(0)
        .expect("Expected a single numeric literal (#[packet_id(0x00)]");

    let packet_id: i32 = {
        let lit: &syn::LitInt = &attribute.parse_args().unwrap();
        let n: i32 = lit.base10_parse().expect("Expected a single numeric literal");
        n
    };

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);

                let gen = quote! {
                    impl #struct_name {
                        #[inline(always)]
                        fn packet_id(&self) -> crate::types::VarInt {
                            crate::types::VarInt(#packet_id)
                        }
                    }

                    impl crate::encoder::SendToWriter for #struct_name {
                        async fn send<W>(&self, stream: &mut W) -> Result<(), crate::errors::EncodeError>
                        where
                            W: AsyncWrite + Unpin {
                            let mut buffer = vec![];

                            #(self.#field_names.encode(&mut buffer).await?;)*
                            let buffer = crate::utils::prepare_response(crate::types::VarInt(#packet_id), buffer).await;
                            Ok(stream.write_all(&buffer).await?)
                        }
                    }
                };

                gen.into()
            }
            _ => panic!("Expected a named field"),
        },
        _ => panic!("Expected a struct"),
    }
}

#[proc_macro_derive(Receivable)]
pub fn derive_receivable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident.clone().unwrap();
                    let field_type = &field.ty;

                    quote! {
                        #field_name: <#field_type>::decode(cursor).await?,
                    }
                });

                let gen = quote! {
                    impl crate::decoder::ReceiveFromStream for #struct_name {
                        async fn receive(cursor: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::errors::DecodeError> {
                            Ok(Self { #(#fields)* })
                        }
                    }
                };

                gen.into()
            }
            _ => panic!("Expected a named field"),
        },
        _ => panic!("Expected a struct with named fields"),
    }
}
