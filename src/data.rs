use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55\" 4K UHD Smart TV".to_string(),
            price: 649.99,
            description: "Experience cinematic brilliance with the Samsung 55\" Crystal UHD 4K Smart TV. Features built-in Alexa, HDR, and dynamic crystal color for vivid visuals.".to_string(),
            image: "/samsung_tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple AirPods Pro (2nd Gen)".to_string(),
            price: 249.99,
            description: "Immerse yourself in rich audio with Apple AirPods Pro. Features active noise cancellation, adaptive transparency, and personalized spatial audio.".to_string(),
            image: "/airpods_pro.jpeg".to_string()
        },
        Product {
            id: 3,
            name: "HP Pavilion x360 2-in-1 Laptop".to_string(),
            price: 799.99,
            description: "Stay productive with the HP Pavilion x360. This 2-in-1 touchscreen laptop offers Intel Core i5 performance and sleek portability for everyday computing.".to_string(),
            image: "/hp_x360.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Sony PlayStation 5 Console".to_string(),
            price: 699.99,
            description: "Next-level gaming with the PlayStation 5. Ultra-fast SSD, ray tracing graphics, and DualSense controller for immersive play.".to_string(),
            image: "/ps5.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Bose QuietComfort 45 Headphones".to_string(),
            price: 429.99,
            description: "Enjoy world-class noise cancellation and comfort with Bose QC45 headphones. Ideal for travel, work, and immersive listening.".to_string(),
            image: "/bose_qc45.jpeg".to_string()
        },
        Product {
            id: 6,
            name: "Nikon D5600 DSLR Camera Kit".to_string(),
            price: 899.99,
            description: "Capture stunning moments with the Nikon D5600 DSLR. Includes 18-55mm lens, built-in Wi-Fi, and 24.2MP resolution for high-quality photography.".to_string(),
            image: "/nikon_d5600.jpg".to_string()
        },
        Product {
            id: 7,
            name: "iRobot Roomba j7+ Self-Emptying Vacuum".to_string(),
            price: 999.99,
            description: "Smart home cleaning with the Roomba j7+. Avoids obstacles and empties itself for a hands-free vacuuming experience.".to_string(),
            image: "/roomba_j7.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Fitbit Charge 6 Fitness Tracker".to_string(),
            price: 219.99,
            description: "Track your health and fitness goals with the Fitbit Charge 6. Includes built-in GPS, heart rate monitor, and Google Maps integration.".to_string(),
            image: "/fitbit_charge6.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Google Nest Thermostat".to_string(),
            price: 139.99,
            description: "Save energy and stay comfortable with the Google Nest Thermostat. Learns your habits and helps reduce heating and cooling costs.".to_string(),
            image: "/nest_thermostat.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Anker PowerCore 20,000mAh Power Bank".to_string(),
            price: 49.99,
            description: "Charge your devices on the go with the Anker PowerCore. Delivers fast, reliable charging with dual USB ports and long-lasting capacity.".to_string(),
            image: "/anker_powerbank.jpeg".to_string()
        }
        
    ]
}