#[derive(Clone, Copy)]
pub struct NavLink {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Clone, Copy)]
pub struct Product {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub image: &'static str,
}

#[derive(Clone, Copy)]
pub struct Spec {
    pub label: &'static str,
    pub value: &'static str,
}

pub const PRIMARY_NAV: &[NavLink] = &[
    NavLink { label: "Atelier", href: "#atelier" },
    NavLink { label: "Calibro", href: "#calibro" },
    NavLink { label: "Collezione", href: "#collezione" },
];

pub const UTILITY_NAV: &[NavLink] = &[
    NavLink { label: "Cerca", href: "#" },
    NavLink { label: "Account", href: "#" },
    NavLink { label: "Carrello\u{00a0}(0)", href: "#" },
];

pub const FOOTER_NAV: &[NavLink] = &[
    NavLink { label: "L'Atelier", href: "#atelier" },
    NavLink { label: "Il Calibro", href: "#calibro" },
    NavLink { label: "La Collezione", href: "#collezione" },
    NavLink { label: "Storia", href: "#" },
    NavLink { label: "Contatti", href: "#" },
    NavLink { label: "Stampa", href: "#" },
];

pub const PRODUCTS: &[Product] = &[
    Product {
        name: "Foglio Notturno",
        subtitle: "Cordino nero · pomello in ottone",
        image: "/products/foil-noir.png",
    },
    Product {
        name: "Foglio Laguna",
        subtitle: "Seta verde acqua · pomello satinato",
        image: "/products/foil-laguna.png",
    },
    Product {
        name: "Impugnature in cordino",
        subtitle: "Sei colori · filato di cotone egiziano",
        image: "/products/impugnature-cordino.png",
    },
];

pub const CALIBRO_SPECS: &[Spec] = &[
    Spec { label: "Lunghezza totale", value: "110 cm" },
    Spec { label: "Peso", value: "480 g" },
    Spec { label: "Punto d'equilibrio", value: "17,5 cm dalla guardia" },
    Spec { label: "Lama", value: "Acciaio maraging, FIE omologata" },
    Spec { label: "Impugnatura", value: "Cordino italiano, sei colori" },
    Spec { label: "Pomello", value: "Ottone tornito a mano" },
];
