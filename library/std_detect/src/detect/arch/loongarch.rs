//! Run-time feature detection on LoongArch.

features! {
    @TARGET: loongarch;
    @CFG: any(target_arch = "loongarch32", target_arch = "loongarch64");
    @MACRO_NAME: is_loongarch_feature_detected;
    @MACRO_ATTRS:
    /// Checks if `loongarch` feature is enabled.
    /// Supported arguments are:
    ///
    /// * `"f"`
    /// * `"d"`
    /// * `"frecipe"`
    /// * `"div32"`
    /// * `"lsx"`
    /// * `"lasx"`
    /// * `"lam-bh"`
    /// * `"lamcas"`
    /// * `"ld-seq-sa"`
    /// * `"scq"`
    /// * `"lbt"`
    /// * `"lvz"`
    /// * `"ual"`
    #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")]
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] f: "f";
    /// F
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] d: "d";
    /// D
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] frecipe: "frecipe";
    /// Frecipe
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] lsx: "lsx";
    /// LSX
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] lasx: "lasx";
    /// LASX
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] lbt: "lbt";
    /// LBT
    @FEATURE: #[stable(feature = "stdarch_loongarch_feature", since = "1.89.0")] lvz: "lvz";
    /// LVZ
    @FEATURE: #[unstable(feature = "stdarch_loongarch_feature_detection", issue = "117425")] ual: "ual";
    /// UAL
}
