use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "TechEdge Smart Hub".to_string(),
            price: 149.99,
            description: "Centralize your smart home devices with the TechEdge Smart Hub. Easy setup, voice assistant compatibility, and seamless connectivity for your smart ecosystem.".to_string(),
            image: "/smarthub.jpg".to_string()
        },
        Product {
            id: 2,
            name: "SonicWave Noise-Canceling Headphones".to_string(),
            price: 199.99,
            description: "Immerse yourself in your favorite music with SonicWave Noise-Canceling Headphones. Exceptional sound quality, long battery life, and comfortable design.".to_string(),
            image: "/sonicwave.jpg".to_string()
        },
        Product {
            id: 3,
            name: "CloudTouch 15\" Laptop".to_string(),
            price: 999.99,
            description: "Work and play effortlessly with the CloudTouch 15\" Laptop. Equipped with a fast processor, vibrant display, and long-lasting battery.".to_string(),
            image: "/cloudtouch.jpg".to_string()
        },
        Product {
            id: 4,
            name: "ChargePro Ultra Power Bank".to_string(),
            price: 49.99,
            description: "Stay powered up on the go with the ChargePro Ultra Power Bank. Features fast charging, multiple ports, and a sleek design.".to_string(),
            image: "/powerbank.jpg".to_string()
        },
        Product {
            id: 5,
            name: "VRX 360 VR Headset".to_string(),
            price: 349.99,
            description: "Dive into immersive virtual experiences with the VRX 360 VR Headset. High-definition visuals and advanced motion tracking for unparalleled realism.".to_string(),
            image: "/headset.jpg".to_string()
        },
        Product {
            id: 6,
            name: "PixelSharp 4K Gaming Monitor".to_string(),
            price: 599.99,
            description: "Take your gaming to the next level with the PixelSharp 4K Gaming Monitor. Stunning visuals, high refresh rates, and ergonomic design.".to_string(),
            image: "/monitor.jpeg".to_string()
        },
        Product {
            id: 7,
            name: "EchoTone Smart Speaker".to_string(),
            price: 79.99,
            description: "Enjoy high-quality audio and smart assistant features with the EchoTone Smart Speaker. Sleek design and seamless integration with your devices.".to_string(),
            image: "/speaker.jpeg".to_string()
        }
    ]
}
