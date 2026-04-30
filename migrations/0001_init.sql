-- Newsletter (preserved across the migration switch — IF NOT EXISTS keeps any
-- row that landed during the pre-migration CREATE TABLE phase).
CREATE TABLE IF NOT EXISTS newsletter_subscribers (
    email         TEXT PRIMARY KEY NOT NULL,
    subscribed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE products (
    slug           TEXT PRIMARY KEY,
    name           TEXT NOT NULL,
    subtitle       TEXT NOT NULL,
    kind           TEXT NOT NULL CHECK (kind IN ('bespoke','catalog')),
    price_cents    INTEGER,
    currency       TEXT NOT NULL DEFAULT 'EUR',
    stock          INTEGER,
    description_md TEXT NOT NULL,
    hero_image     TEXT NOT NULL,
    published      INTEGER NOT NULL DEFAULT 1,
    sort_order     INTEGER NOT NULL DEFAULT 0,
    created_at     TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE product_images (
    product_slug TEXT NOT NULL REFERENCES products(slug) ON DELETE CASCADE,
    position     INTEGER NOT NULL,
    src          TEXT NOT NULL,
    alt          TEXT NOT NULL,
    PRIMARY KEY (product_slug, position)
);

CREATE TABLE product_specs (
    product_slug TEXT NOT NULL REFERENCES products(slug) ON DELETE CASCADE,
    position     INTEGER NOT NULL,
    label        TEXT NOT NULL,
    value        TEXT NOT NULL,
    PRIMARY KEY (product_slug, position)
);

CREATE TABLE inquiries (
    id           INTEGER PRIMARY KEY,
    product_slug TEXT REFERENCES products(slug),
    email        TEXT NOT NULL,
    name         TEXT NOT NULL,
    message      TEXT NOT NULL,
    locale       TEXT NOT NULL DEFAULT 'it',
    created_at   TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    handled_at   TEXT
);

CREATE INDEX idx_inquiries_unhandled
    ON inquiries(created_at)
    WHERE handled_at IS NULL;

-- Seed: 3 SKUs from the original static content (all bespoke, NULL price/stock).
INSERT INTO products (slug, name, subtitle, kind, description_md, hero_image, sort_order) VALUES
    ('foglio-notturno',
     'Foglio Notturno',
     'Cordino nero · pomello in ottone',
     'bespoke',
     'Foglio italiano da pedana, lama maraging temprata in olio. Cordino in cotone egiziano nero teso a mano sopra impugnatura in legno di noce. Pomello in ottone, tornito uno alla volta nella bottega di Via Mazzini. Bilanciamento italiano, vicino alla guardia. Omologato FIE.',
     '/products/foil-noir.png',
     1),
    ('foglio-laguna',
     'Foglio Laguna',
     'Seta verde acqua · pomello satinato',
     'bespoke',
     'La stessa lama del Foglio Notturno, ma con cordino in seta verde acqua e pomello in acciaio satinato. Pensato per l''allenamento e per chi preferisce una linea più sobria. Omologato FIE.',
     '/products/foil-laguna.png',
     2),
    ('impugnature-cordino',
     'Impugnature in cordino',
     'Sei colori · filato di cotone egiziano',
     'bespoke',
     'Impugnature di ricambio in cordino di cotone egiziano, intrecciate e cucite a mano. Sei colori disponibili: nero, blu Verona, verde laguna, rosso ottone, avorio, grigio antracite. Si adattano a qualsiasi foglio italiano standard.',
     '/products/impugnature-cordino.png',
     3);

-- Seed: the 6 calibro specs, attached to foglio-notturno (the GO·01 reference).
INSERT INTO product_specs (product_slug, position, label, value) VALUES
    ('foglio-notturno', 1, 'Lunghezza totale',     '110 cm'),
    ('foglio-notturno', 2, 'Peso',                  '480 g'),
    ('foglio-notturno', 3, 'Punto d''equilibrio',   '17,5 cm dalla guardia'),
    ('foglio-notturno', 4, 'Lama',                  'Acciaio maraging, FIE omologata'),
    ('foglio-notturno', 5, 'Impugnatura',           'Cordino italiano, sei colori'),
    ('foglio-notturno', 6, 'Pomello',               'Ottone tornito a mano');
