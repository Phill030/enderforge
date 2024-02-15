use crate::encoder::Encoder;
use crate::encoder::SendToStream;
use crate::utils::system_time_millis;
use macros::Streamable;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use uuid::Uuid;

const BASE64_ICON: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAIGNIUk0AAHomAACAhAAA+gAAAIDoAAB1MAAA6mAAADqYAAAXcJy6UTwAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAACxMAAAsTAQCanBgAAAAHdElNRQfoAQQCDQ7VUwqMAAAAKWlUWHRkYXRlOmNyZWF0ZQAAAAAAMjAyNC0wMS0wNFQwMjoxMzoxMCswMDowMJZqSgwAAAApaVRYdGRhdGU6bW9kaWZ5AAAAAAAyMDI0LTAxLTA0VDAyOjEzOjEwKzAwOjAwIpR2NQAAACxpVFh0ZGF0ZTp0aW1lc3RhbXAAAAAAADIwMjQtMDEtMDRUMDI6MTM6MTQrMDA6MDALvmyVAAAAMXRFWHRDb21tZW50AFBORyByZXNpemVkIHdpdGggaHR0cHM6Ly9lemdpZi5jb20vcmVzaXplXknb4gAAABJ0RVh0U29mdHdhcmUAZXpnaWYuY29toMOzWAAACjhJREFUaN7tmWlsXcUVx/9nZu59m/28xbtjOyGxYyBJnUADJJAVQmiBJkFNUaum6pdKSFQgoBUEGipU2lKkVipSQa1U1C2tWmggbQIpoYEkju1EQUBCNse7493v2c/v3XeXmekHOwmLl5etViSfT1dv7p35/+acM2dmHsnXmnAtG5tqAdMAUy1gGmCqBUwDTLWAaYCpFjANMNUCpgGmWsA1AEDXFgCBOGhENCnPVl7yHAOB+BXnuSIAdF6xdGLWqX+6w70gppRrHf6VVf8LJR0Q8xJ91snXpT04ykB0RWAuH4CkG3eGujQIjLudh1DzQ6f+p54dszvq+Ok/84Ztdtt+z47bh15AzQ+cjoNgXIOcWJd0hi+fgW/d9MilTzwA4vbpN2XN05KHKFzuHXtV9H7Ahk55kUa0vs2HW0m5Ktoguw6J1n8x6UoeYEXLnMadsuZJxf1G3pcAfTkY4tI+00pK6QozqAE1cNyInlL1zySbd4n+I2AgLUXrrvOMInIMkWMggIF3vJPcExG9dYYdcwdOamgi5jkJxg3G+P8LgLjVfkAd/R0vX8uKbqXBBjAwL252/OfCVH56Tj/1zJyIefadERg2eNoZaFRd9bJpJ7t+c3DWGmh5tQEI0CCS8W6jay/17JOBYm73nF9mUu0DAIH1HZG7v82tTiY9r3QtiKBH2vTVApBuAsSYaUKYIEZainjrJQcwKVvE20BQjJTwazDlDkNLboauCoD07ETt82y4lXKrabCJQV2Yzks2AgAGTa27rVgzeo/IQFFg2bPCCKboh4vxADFudYmO99H5/hWQ/lkz299C+1tQQMlyIp56FKVWB4iBCWaEkLNQM+DKlKDPDYGRzEbOAmamgQlQStpS8oDbf1LFmjn36UT3lRb+GdMA4p1u81vSc1j6TF/uDZODT3q1qLW29j3Fz/yVmAEtodVVZQBIE4fy5Kz1/uUvMjaJHyZ3EzFBBbdoMqDcq68egCbtgQkqXMq4MenbkwEQAzEx8w4VnnMxq/NlIkCmz+Yzl4+MPvG7E+eAdqPNMnJaRRqYE0lxdAIYQQPqi8AanKBprKbPdkF2v/fJH7yseSK70siaTTTuojFRDniebb37qNnxNrSiVJcdipnhM/3DYSbLg+B0wW0EJH3hE1E7rO1ZwZT8oAGncFVg9UvCHLcsTOQgLvy85A7NRKrqFVTVUrZ1559mP3j3QbzSAnluUNLoTivxHttWu+bpDYf59s4U1mGC5iYvXcXNiYoam7AH+CrWy5K7U4x+W6J9zoqcBbfdvvbeNpv9/DSOxcAJAOIudhqV6UvuuWPtV9sp/MsziLiTMWio4jt9lQ9M/NqEKUKkrAFK9qeingF9Dn6z50hDY1Nt7UFo1WPj46HRlOhxse1Qw/739u7Z844Vj52OoykBNqkX3CE53DlxHo+fxMTc4W6n5lmj+0BKdZcgCW/s2v16/are7u5zM3AOj6GlpWXjxo2OnZSe53BYk26cCaJzn/fuQ3LR4/6ylcQY9BiRMA4cMTfRb9c+Jzp2g1LaOmiNfD+WZXitzc2WZQEoCWBhGApQGoU+rM5FNDKQSCQALMrE3NBEa9HIiEQQ0eO0/zHryEueNTimijGPlOQlo07dT4yW7URQoKgNV8NkkzjdZFicCaUx6KEqDVsqsCxnVIrBsCADwx7SBR4owpYKlAbGzSxPod9GQplMS87BVJK6a93hXlGyjLj5uZfHCiFiXtPbovG1kQBIlm482tQc6zhckBEuMobyAmNgjPww5EIwbKnAE3MQ4AgLuBquhq2QVMgy8HwVOpJIF5hhjvG5Pvfc5QZbs1ZGOz7KTg8vnBEO9B4kaEp06rFCaMwc0JQ9T4aKRbxdA2Zm2dwVG+v+8XAkc4G/eJ6ve0eW2wbAlmAEg4ER+hzs70dTAq6CJVESwD35yDCw/Sx+34aYC1fj/gJUpCHLQMTFJzEU+DE/HZxBKiQ8DLnINBHicAMFGbc9M9Pz9bUcLrz1YRHdh56DygxR1WbuS/viXmbMENI8rcALFuuuOuYmVLI/vGAzD+X3NPz3+lWPheesU1313I702nR00D+QlJ1JHI2hPIjVM7A0GwszEfOwoxuZBq4L4dgQOpIIC2woxJ15qEhDWRClAXTbOBFDwkZj3Gy1fBnczfFDG2lYvCVUed/x/b8N58+bV30fjr7CrF5vzoP+GzePWY/HvlYhQGTNcf1FuruOx9ulyMhb9J3YQMdA59Gy6q/LYJHqPBAiixXfniy7P9J/ZmlGojw4miRBjso0lAbwZhfSOO6agTwTm4qwIgeCoDU0wAlFfnQ7rC9/XfGcFWW8p4hHtJmpqx8PXP+N/s5P2k+9t+jOJ8TgCTrxqsy+0bxlqwhmjVnOxr0XIkDkVLr+QnTXUeQYFS3Lq7yr/dQ+05+WPXu5J7Kp51Cm3ZxjqnKcDcBRGC3+mjFFRkYwvSQj3JZ/v2/uAxl+88tl5TpjrhfK93iQtCLlALoogOIg8oY/DCXbvMwq3Pwjf8XXADp55PWSucvzZs53PnwZQ4205Me+gurxNsKTnAc0yGr4N+qeQe7N/pW/Gox2tp6uuXHJJs6F1bATH7zIh5pGFhoNcvOW8tI1KpBr+tKkmQPDb6YVMF+6chPEBEBKecq1nI46Vvc0d/pGBpCBPD1rvVH1LSOzjJGO9LW1n6mtummDF2n0dm/G7PX+mx9l49eySU5kBB2Y8xWLmD70XPLYH7Oqv+cqPTDYFwiGWelqGZ7tNO7wtWznibOAZol2bYR8Zau4L2wSCBojLvGlnfOOKYe76Mxr3OkHwAC34Da66SmRU6mBpB2XWiccp6xqJWPcbXgD4dm++d9ljI9ZwlICGGW4bp1FJD962S5cUlC0+G/vb//1zldNYQhh+s3ADTL0/Rzk+CBizbr2Kbvxjc60BfVDvN1CQo4EIzih0K/vyHRm9r8noieIQISaiPh7xEy2/GUwHolZcU96UsonNz60dnGFHW1VfR+b1Y+I4IyJb7tSOhMTdGD2Oktru/ldkTOvvuHDAycOn2/dAXSV4Ll5yPUB2jW69heq/bcmse0sfnYKBsM3i7EsC/PDKApBcGjAktjeha3Hvdbk3s+NVZCVu6TyJqP/BJWuMQsXT3pXN/mZ+EI+aJ3oOHw0Yr2wa1s0Hg36AjPSs9MDIZ/h8wlxT56ujtewyMektCbo7Pknix486YUFsRtCXvlQPboPKmcIIqiz5/dmLt4fNduGhhLJuCc923Utx+qLRaLxIaXUpqX3bqi8LiOvQgSzJgieiwYAoDWGk8Naa0MYgnHBBSNGRCAoTW7srNe2Vw6eYcF8UXaXP6t8dDen4XmOtPqkM8xFgAdzmeFjRKOJAQ2tldZSSU96rufZnpPuD5mGL5XboYsDADBaTcYs60QA01rTqDj1hdaRb9Wk/evJJv68XfTt9ERdaw1IwjgTp3UqM5q69BGb/pdyqm0aYKptGmCqbRpgqm0aYKptGmCq7ZoH+B8wyJrP3MElbAAAAABJRU5ErkJggg==";

#[derive(Streamable)]
#[packet_id(0x00)]
pub struct PlayerListResponse {
    json_response: String,
}

impl PlayerListResponse {
    pub fn new(version_name: String, version_protocol: u32, max_players: u32, motd: String) -> Self {
        let list = PlayerList {
            version: Version {
                name: version_name,
                protocol: version_protocol,
            },
            players: Players {
                max: max_players,
                online: 0,
                sample: vec![Player {
                    name: format!("{}", Uuid::new_v4()),
                    id: format!("{}", Uuid::new_v4()),
                }],
            },
            description: Description { text: motd },
            enforces_secure_chat: false,
            previews_chat: false,
            favicon: Some(BASE64_ICON.to_string()),
        };

        Self {
            json_response: serde_json::to_string(&list).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerList {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
    #[serde(rename = "previewsChat")]
    pub previews_chat: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<Player>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}

#[derive(Streamable)]
#[packet_id(0x01)]
pub struct PingResponse {
    system_time_millis: u64,
}

impl PingResponse {
    #[must_use]
    pub fn new() -> Self {
        Self {
            system_time_millis: system_time_millis().unwrap(),
        }
    }
}

pub struct Status;
impl Status {
    pub async fn handle(stream: &mut TcpStream) {
        PingResponse::new().send(stream).await.unwrap();
    }
}
