use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

/// African currencies tracked by the FX grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    // West African CFA franc (UEMOA)
    XOF,
    // Central African CFA franc (CEMAC)
    XAF,
    // Nigerian naira
    NGN,
    // Ghanaian cedi
    GHS,
    // Kenyan shilling
    KES,
    // Ugandan shilling
    UGX,
    // Tanzanian shilling
    TZS,
    // Rwandan franc
    RWF,
    // South African rand
    ZAR,
    // Ethiopian birr
    ETB,
    // Egyptian pound
    EGP,
    // Moroccan dirham
    MAD,
    // Reference currencies
    USD,
    EUR,
    // Crypto on/off ramps
    USDT,
    BTC,
}

impl Currency {
    /// Returns the ISO 4217 code for the currency.
    pub fn code(&self) -> &str {
        match self {
            Currency::XOF => "XOF",
            Currency::XAF => "XAF",
            Currency::NGN => "NGN",
            Currency::GHS => "GHS",
            Currency::KES => "KES",
            Currency::UGX => "UGX",
            Currency::TZS => "TZS",
            Currency::RWF => "RWF",
            Currency::ZAR => "ZAR",
            Currency::ETB => "ETB",
            Currency::EGP => "EGP",
            Currency::MAD => "MAD",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::USDT => "USDT",
            Currency::BTC => "BTC",
        }
    }

    /// Returns the currency's display name.
    pub fn name(&self) -> &str {
        match self {
            Currency::XOF => "West African CFA franc",
            Currency::XAF => "Central African CFA franc",
            Currency::NGN => "Nigerian naira",
            Currency::GHS => "Ghanaian cedi",
            Currency::KES => "Kenyan shilling",
            Currency::UGX => "Ugandan shilling",
            Currency::TZS => "Tanzanian shilling",
            Currency::RWF => "Rwandan franc",
            Currency::ZAR => "South African rand",
            Currency::ETB => "Ethiopian birr",
            Currency::EGP => "Egyptian pound",
            Currency::MAD => "Moroccan dirham",
            Currency::USD => "US Dollar",
            Currency::EUR => "Euro",
            Currency::USDT => "Tether USD",
            Currency::BTC => "Bitcoin",
        }
    }

    /// Returns the monetary zone the currency belongs to.
    pub fn zone(&self) -> MonetaryZone {
        match self {
            Currency::XOF => MonetaryZone::UEMOA,
            Currency::XAF => MonetaryZone::CEMAC,
            Currency::NGN | Currency::GHS => MonetaryZone::WAMZ,
            Currency::KES | Currency::UGX | Currency::TZS | Currency::RWF => MonetaryZone::EAC,
            Currency::ZAR => MonetaryZone::SADC,
            Currency::ETB | Currency::EGP | Currency::MAD => MonetaryZone::Other,
            Currency::USD | Currency::EUR => MonetaryZone::Global,
            Currency::USDT | Currency::BTC => MonetaryZone::Crypto,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Monetary zones corresponding to African economic regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonetaryZone {
    /// Union Economique et Monetaire Ouest Africaine
    UEMOA,
    /// Communaute Economique et Monetaire de l'Afrique Centrale
    CEMAC,
    /// West African Monetary Zone
    WAMZ,
    /// East African Community
    EAC,
    /// Southern African Development Community
    SADC,
    /// Other African
    Other,
    /// Global reference currencies
    Global,
    /// Cryptocurrency
    Crypto,
}

/// A directed exchange rate between two currencies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub from: Currency,
    pub to: Currency,
    pub rate: Decimal,
    pub source: RateSource,
    pub timestamp_ns: i64,
    pub confidence: f64, // 0.0 to 1.0
}

/// Where the rate originated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateSource {
    CentralBank { country: String },
    MobileMoney { provider: String, country: String },
    Parallel { market: String, verified_by: Vec<String> },
    CryptoExchange { exchange: String },
    Crowdsourced { submitter: String, attestations: u32 },
}
