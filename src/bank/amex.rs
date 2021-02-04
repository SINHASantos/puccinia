use bank_ofx;

bank_ofx!("amex", Amex, {
    fn url(&self) -> &str {
        "https://online.americanexpress.com/myca/ofxdl/desktop/desktopDownload.do?request_type=nl_ofxdownload"
    }

    fn fid(&self) -> &str {
        "3101"
    }

    fn fid_org(&self) -> &str {
        "AMEX"
    }

    fn app_id(&self) -> &str {
        "QWIN"
    }

    fn app_ver(&self) -> &str {
        "1500"
    }
});
