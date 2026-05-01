-- Orders + line items. Stripe is the source of truth for payment state;
-- we mirror it locally so we can render order confirmations and decrement
-- stock without round-tripping Stripe.

CREATE TABLE orders (
    id                    TEXT PRIMARY KEY,                       -- ULID
    stripe_session_id     TEXT UNIQUE,
    stripe_payment_intent TEXT UNIQUE,
    status                TEXT NOT NULL CHECK (status IN
                              ('pending','paid','failed','fulfilled','refunded')),
    email                 TEXT NOT NULL,
    subtotal_cents        INTEGER NOT NULL,
    shipping_cents        INTEGER NOT NULL,
    total_cents           INTEGER NOT NULL,
    currency              TEXT NOT NULL DEFAULT 'EUR',
    shipping_address_json TEXT,
    locale                TEXT NOT NULL DEFAULT 'en',
    created_at            TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    paid_at               TEXT
);

CREATE TABLE order_items (
    order_id          TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_slug      TEXT NOT NULL REFERENCES products(slug),
    name_snapshot     TEXT NOT NULL,
    unit_price_cents  INTEGER NOT NULL,
    quantity          INTEGER NOT NULL,
    PRIMARY KEY (order_id, product_slug)
);

CREATE INDEX idx_orders_status_created ON orders(status, created_at DESC);

-- Seed: 3 catalog-kind cordino accessory SKUs (cart-purchasable).
-- Stripe Tax (when enabled in dashboard) handles IVA on top of these prices.
INSERT INTO products (slug, name, subtitle, kind, price_cents, stock,
                      description_md, hero_image, sort_order) VALUES
    ('cordino-rosso',
     'Cordino Rosso',
     'Brass red · Egyptian cotton',
     'catalog',
     4500,
     8,
     'Replacement cord grip in brass-red Egyptian cotton, woven and stitched by hand. Fits any standard Italian foil. Ships from Verona.',
     '/products/impugnature-cordino.png',
     10),
    ('cordino-blu',
     'Cordino Blu Verona',
     'Verona blue · Egyptian cotton',
     'catalog',
     4500,
     8,
     'Replacement cord grip in Verona blue Egyptian cotton, woven and stitched by hand. Fits any standard Italian foil. Ships from Verona.',
     '/products/impugnature-cordino.png',
     11),
    ('cordino-verde',
     'Cordino Verde Laguna',
     'Lagoon green · Egyptian cotton',
     'catalog',
     4500,
     8,
     'Replacement cord grip in lagoon-green Egyptian cotton, woven and stitched by hand. Fits any standard Italian foil. Ships from Verona.',
     '/products/impugnature-cordino.png',
     12);
