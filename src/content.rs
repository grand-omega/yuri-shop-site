#[derive(Clone, Copy)]
pub struct NavLink {
    pub label: &'static str,
    pub href: &'static str,
}

pub const PRIMARY_NAV: &[NavLink] = &[
    NavLink { label: "Atelier", href: "/#atelier" },
    NavLink { label: "Caliber", href: "/#calibro" },
    NavLink { label: "Collection", href: "/collezione" },
];

pub const UTILITY_NAV: &[NavLink] = &[
    NavLink { label: "Search", href: "#" },
    NavLink { label: "Account", href: "#" },
    NavLink { label: "Cart\u{00a0}(0)", href: "#" },
];

pub const FOOTER_NAV: &[NavLink] = &[
    NavLink { label: "The Atelier", href: "/#atelier" },
    NavLink { label: "The Caliber", href: "/#calibro" },
    NavLink { label: "The Collection", href: "/collezione" },
    NavLink { label: "Heritage", href: "#" },
    NavLink { label: "Contact", href: "#" },
    NavLink { label: "Press", href: "#" },
];
