
pub const OKP4_NETWORK: NetworkInfo = NetworkInfo {
    id: "okp4-localnet",
    pub_address_prefix: "okp4",
    coin_type: 118u32, // TBD
};

pub const LOCAL_OKP4: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "okp4-localnet",
    gas_denom: "uknow",
    gas_price: 0.0,
    grpc_urls: &["http://localhost:9090"],
    network_info: OKP4_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
