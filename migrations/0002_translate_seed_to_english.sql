-- Translate seeded copy from Italian to English. Product names stay Italian
-- as proper-noun product references (Foglio Notturno, Foglio Laguna,
-- Impugnature in cordino) — only subtitles, descriptions, spec labels/values,
-- and the inquiries.locale default flip to English.

UPDATE products SET
    subtitle       = 'Black cord · brass pommel',
    description_md = 'Italian competition foil, maraging blade tempered in oil. Black Egyptian cotton cord drawn by hand over a walnut grip. Brass pommel, lathe-turned one at a time in our workshop on Via Mazzini. Italian balance, close to the guard. FIE-homologated.'
WHERE slug = 'foglio-notturno';

UPDATE products SET
    subtitle       = 'Sea-green silk · satin pommel',
    description_md = 'The same blade as the Foglio Notturno, with a sea-green silk cord and a satin steel pommel. For training, and for those who prefer a quieter line. FIE-homologated.'
WHERE slug = 'foglio-laguna';

UPDATE products SET
    subtitle       = 'Six colours · Egyptian cotton',
    description_md = 'Replacement cord grips in Egyptian cotton, woven and stitched by hand. Six colours: black, Verona blue, lagoon green, brass red, ivory, charcoal grey. Fits any standard Italian foil.'
WHERE slug = 'impugnature-cordino';

UPDATE product_specs SET label = 'Total length',  value = '110 cm'                          WHERE product_slug = 'foglio-notturno' AND position = 1;
UPDATE product_specs SET label = 'Weight',        value = '480 g'                           WHERE product_slug = 'foglio-notturno' AND position = 2;
UPDATE product_specs SET label = 'Balance point', value = '17.5 cm from guard'              WHERE product_slug = 'foglio-notturno' AND position = 3;
UPDATE product_specs SET label = 'Blade',         value = 'Maraging steel, FIE-homologated' WHERE product_slug = 'foglio-notturno' AND position = 4;
UPDATE product_specs SET label = 'Grip',          value = 'Italian cord, six colours'       WHERE product_slug = 'foglio-notturno' AND position = 5;
UPDATE product_specs SET label = 'Pommel',        value = 'Lathe-turned brass'              WHERE product_slug = 'foglio-notturno' AND position = 6;
