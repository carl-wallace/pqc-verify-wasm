#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::debug;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, FileReader};
#[derive(Props, PartialEq)]
pub struct SettingsProps {
    pub x: String,
}

use certval::{
    is_self_signed, populate_5280_pki_environment, Error, PDVCertificate, PkiEnvironment, Result,
};
use log::error;

pub fn get_file_as_byte_vec_pem(buf: &[u8]) -> Result<PDVCertificate> {
    let buf = if buf[0] == 0x2D {
        match pem_rfc7468::decode_vec(buf) {
            Ok(b) => b.1,
            Err(e) => {
                error!("Failed to PEM decode data: {:?}", e);
                return Err(Error::Unrecognized);
            }
        }
    } else {
        buf.to_vec()
    };
    debug!("Attempting to parse buffer as Certificate");
    Ok(PDVCertificate::try_from(buf.as_slice())?)
}

pub(crate) fn App(cx: Scope<'_>) -> Element<'_> {
    let pqc_zip_file = "".to_string();
    let s_pqc_zip_file = use_state(cx, || pqc_zip_file);

    let pqc_zip_contents = vec![];
    let s_pqc_zip_contents = use_state(cx, || pqc_zip_contents);

    let status_label = "Choose the Choose File button".to_string();
    let s_status_label = use_state(cx, || status_label);

    cx.render(rsx! {
        div {
            form {
                onsubmit: move |_| {
                    let label_setter = s_status_label.setter();
                    to_owned![s_pqc_zip_contents];
                    async move {
                        match get_file_as_byte_vec_pem(s_pqc_zip_contents.as_slice()) {
                            Ok(ta) => {
                                let mut pe = PkiEnvironment::default();
                                populate_5280_pki_environment(&mut pe);

                                match is_self_signed(&pe, &ta) {
                                    true => {
                                        debug!("Is self-signed");
                                        label_setter("Is self-signed".to_string());
                                    },
                                    false => {
                                        debug!("Is not self-signed");
                                        label_setter("Is not self-signed".to_string());
                                    }
                                };
                            }
                            Err(e) => {
                                debug!("Failed to parse file as a trust anchor: {:?}", e);
                                label_setter("Failed to parse file as a trust anchor".to_string());
                            }
                        };
                    }
                },
                oninput: move |ev| println!("Input {:?}", ev.values),
                fieldset {
                    table {
                        tbody {
                            table {
                                tbody {
                                    tr{
                                        td{label {r#for: "pqc-cert-file", "PQC Hackathon Self-Signed Certificate File (R3): "}}
                                        input {
                                            r#type: "file",
                                            accept: ".der, .crt",
                                            multiple: false,
                                            directory: false,
                                            onchange: |evt| {
                                                let setter = s_pqc_zip_file.setter();
                                                let bsetter = s_pqc_zip_contents.setter();
                                                let label_setter = s_status_label.setter();
                                                async move {
                                                    debug!("onchange start");
                                                    let filereader = FileReader::new().unwrap();
                                                    //let x = filereader.read_as_data_url(&files_uploaded).unwrap();
                                                    let closure = Closure::wrap(Box::new(move |event: Event| {
                                                        let element = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                                                        let data = element.result().unwrap();
                                                        let js_data = js_sys::Uint8Array::from(data);
                                                        let rust_str: String = js_data.to_string().into();
                                                        debug!("{}", rust_str.as_str());
                                                    }) as Box<dyn FnMut(_)>);

                                                    filereader.set_onloadend(Some(closure.as_ref().unchecked_ref()));
                                                    closure.forget();
                                                    debug!("onchange end");
                                                    if let Some(file_engine) = &evt.files {
                                                        let files = file_engine.files();
                                                        for file_name in files {
                                                            debug!("{file_name}");
                                                            //let s = JsValue::from_str(&file_name);
                                                            //filereader.read_as_data_url(s);
                                                            let x = file_engine.read_file(&file_name).await.unwrap();
                                                            //debug!("STUFF: {:?}", x);
                                                            bsetter(x);
                                                            setter(file_name);
                                                            label_setter("Click the Validate button".to_string())
                                                        }
                                                    }
                                                }
                                            },
                                        },
                                        // td{input { r#type: "text", name: "pqc-cert-file", value: "{s_pqc_zip_file}", style: "width: 500px;"}}
                                        // button {
                                        //     r#type: "button",
                                        //     onclick: move |_| {
                                        //         let setter = s_pqc_zip_file.setter();
                                        //         async move {
                                        //             let file = FileDialog::new()
                                        //                 .add_filter("PQC Hackathon Zip File", &["zip"])
                                        //                 .pick_file();
                                        //             if let Some(file) = file {
                                        //                 setter(file.into_os_string().into_string().unwrap());
                                        //             }
                                        //         }
                                        //     },
                                        //     "..."
                                        // }
                                    }
                                    tr{
                                        td{
                                            label {
                                                "{s_status_label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div{
                    style: "text-align:center",
                    button { r#type: "submit", value: "Submit", "Validate" }
                }
            } //end form
        } //end div
    }) //end render
}
