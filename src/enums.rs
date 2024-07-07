enum DiskType {
    HDD,
    SSD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

// Enum as a type
#[derive(Debug)]
enum WineRegions {
    Napa,
    Sonoma,
    Tuscany,
    Chianti,
    Burgundy,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions, // WineRegions (an enum) used as a type
}

pub fn supported_regions(region: WineRegions) {
    match region {
        WineRegions::Burgundy => println!("Burgundy is supported!"),
        _ => println!("{:?} is not supported!", region),
    }
}

// Option Enum
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None // This is valid because it is the other variant of Option
    } else {
        Some(a / b) // Creates the Option<i32> value. Some() creates a new instance of Option
    }
}

// Applied Enums - real world scenario
#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}

fn format_file_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000), // 999_999 -> The underscores are only for readability
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        _ => FileSize::Gigabytes(size / 1_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kilobytes) => format!("{:.2} KB", kilobytes as f64 / 1000.0),
        FileSize::Megabytes(megabytes) => format!("{:.2} MB", megabytes as f64 / 1000.0),
        FileSize::Gigabytes(gigabytes) => format!("{:.2} GB", gigabytes as f64 / 1000.0),
    }
}

// can also use assciated function
impl FileSize {
    fn format_size(&self) -> String {
        match self {
            Self::Bytes(bytes) => format!("{} bytes", bytes),
            Self::Kilobytes(kilobytes) => format!("{} KB", kilobytes),
            Self::Megabytes(megabytes) => format!("{} MB", megabytes),
            Self::Gigabytes(gigabytes) => format!("{} GB", gigabytes),
        }
    }
}

// Enums with Vectors
#[derive(Debug)]
enum Shape {
    // Variants of Enum
    Circle(f64),        // Radius of a circle
    Square(f64),        // Length of one side for a square
    Triangle(f64, f64), // Base and height for a triangle
}

fn calculate_area() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Square(4.0),
        Shape::Triangle(2.0, 6.0),
    ];

    let total_area = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Triangle(base, height) => 0.5 * base * height,
        })
        .sum::<f64>();

    println!("Total area: {:.2}", total_area);
}

// Exhaustive matches
#[derive(Debug)]
enum WineGrapes {
    CabernetSauvignon,
    Chardonnay,
    Merlot,
    PinotNoir,
}

fn taste_wine(grape: WineGrapes) {
    match grape {
        WineGrapes::Chardonnay => println!("This is a sweet wine."),
        WineGrapes::Merlot => println!("This is a dry wine."),
        _ => println!("This is a regular wine."),
    }
}

pub fn use_enum() {
    let disk_type = DiskType::SSD;
    match disk_type {
        DiskType::HDD => println!("HDD"),
        DiskType::SSD => println!("SSD"),
    }

    let disk_size = DiskSize::GB(256);
    println!("{:?}", disk_size);

    let wine1 = Wine {
        name: "Cabernet Sauvignon".to_string(),
        region: WineRegions::Napa,
    };

    let wine2 = Wine {
        name: "Chardonnay".to_string(),
        region: WineRegions::Burgundy,
    };

    println!("Wine 1: {} from {:?}.", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}.", wine2.name, wine2.region);

    supported_regions(WineRegions::Tuscany);
    supported_regions(wine2.region);

    divide(10, 2).map(|result| println!("Result: {}", result));
    println!("File size: {:?}", format_file_size(979888656554645));

    calculate_area();

    taste_wine(WineGrapes::Chardonnay);
}
