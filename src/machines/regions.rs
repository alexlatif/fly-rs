use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MachineRegions {
    Ams, // Amsterdam, Netherlands
    Arn, // Stockholm, Sweden
    Atl, // Atlanta, Georgia (US)
    Bog, // Bogotá, Colombia
    Bom, // Mumbai, India
    Bos, // Boston, Massachusetts (US)
    Cdg, // Paris, France
    Den, // Denver, Colorado (US)
    Dfw, // Dallas, Texas (US)
    Ewr, // Secaucus, NJ (US)
    Eze, // Ezeiza, Argentina
    Fra, // Frankfurt, Germany
    Gdl, // Guadalajara, Mexico
    Gig, // Rio de Janeiro, Brazil
    Gru, // Sao Paulo, Brazil
    Hkg, // Hong Kong, Hong Kong
    Iad, // Ashburn, Virginia (US)
    Jnb, // Johannesburg, South Africa
    Lax, // Los Angeles, California (US)
    Lhr, // London, United Kingdom
    Mad, // Madrid, Spain
    Mia, // Miami, Florida (US)
    Nrt, // Tokyo, Japan
    Ord, // Chicago, Illinois (US)
    Otp, // Bucharest, Romania
    Phx, // Phoenix, Arizona (US)
    Qro, // Querétaro, Mexico
    Scl, // Santiago, Chile
    Sea, // Seattle, Washington (US)
    Sin, // Singapore, Singapore
    Sjc, // San Jose, California (US)
    Syd, // Sydney, Australia
    Waw, // Warsaw, Poland
    Yul, // Montreal, Canada
    Yyz, // Toronto, Canada
}
