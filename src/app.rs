use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="it" class="h-full antialiased">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="bg-ink text-ivory min-h-full flex flex-col">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/exp-leptos.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400;1,500;1,600&family=EB+Garamond:ital,wght@0,400;0,500;1,400;1,500&display=swap"
        />
        <Title text="Grand Omega — Maître d'armes dal 1897"/>
        <Meta
            name="description"
            content="Atelier di scherma a Verona. Lame, impugnature e pomelli forgiati a mano da quattro generazioni di maestri d'armi."
        />
        <Router>
            <Routes fallback=|| "Not found.".into_view()>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <>
            <header class="fixed top-0 inset-x-0 z-50 bg-ink/92 backdrop-blur-md border-b border-rule">
                <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-5 grid grid-cols-1 gap-5 lg:grid-cols-3 lg:gap-0 items-center">
                    <nav class="flex gap-5 md:gap-7 tracker text-stone justify-center lg:justify-start">
                        <a href="#atelier" class="hover:text-ivory transition-colors duration-500">"Atelier"</a>
                        <a href="#calibro" class="hover:text-ivory transition-colors duration-500">"Calibro"</a>
                        <a href="#collezione" class="hover:text-ivory transition-colors duration-500">"Collezione"</a>
                    </nav>
                    <a href="/" class="font-display text-[1.4rem] md:text-[1.65rem] tracking-[0.22em] text-center text-ivory leading-none">
                        "GRAND\u{00a0}OMEGA"
                    </a>
                    <nav class="hidden lg:flex gap-7 tracker text-stone justify-end">
                        <a href="#" class="hover:text-ivory transition-colors duration-500">"Cerca"</a>
                        <a href="#" class="hover:text-ivory transition-colors duration-500">"Account"</a>
                        <a href="#" class="hover:text-ivory transition-colors duration-500">"Carrello\u{00a0}(0)"</a>
                    </nav>
                </div>
            </header>

            <main class="pt-[122px] lg:pt-[81px] flex-1">
                <section class="relative bg-ink">
                    <div class="mx-auto max-w-[1480px] px-6 md:px-8 pt-20 pb-28 lg:py-32 grid grid-cols-1 lg:grid-cols-12 gap-y-20 lg:gap-x-16 items-center min-h-[calc(100vh-122px)] lg:min-h-[calc(100vh-81px)]">
                        <div class="lg:col-span-7 flex flex-col gap-10 lg:gap-12">
                            <p class="tracker text-stone rise" style="animation-delay: 150ms">
                                "Verona · MMXXVI · Atelier dal MDCCCXCVII"
                            </p>

                            <h1
                                class="font-display font-light text-[clamp(3rem,7.5vw,8.25rem)] leading-[0.95] tracking-[-0.012em] text-ivory rise"
                                style="animation-delay: 350ms"
                            >
                                "L'arte della"
                                <br/>
                                <span class="italic font-normal">"scherma,"</span>
                                <br/>
                                "forgiata a mano."
                            </h1>

                            <div class="h-px w-28 bg-oxblood origin-left draw-x" style="animation-delay: 950ms"></div>

                            <p
                                class="font-serif text-[1.25rem] leading-[1.55] text-stone max-w-[44ch] rise"
                                style="animation-delay: 650ms"
                            >
                                "Maestri d'armi dal 1897. Ogni lama nasce da quattro generazioni di sapere artigiano nella nostra bottega di Via Mazzini."
                            </p>

                            <a
                                href="#collezione"
                                class="group inline-flex items-center gap-4 tracker text-ivory rise w-fit"
                                style="animation-delay: 1050ms"
                            >
                                <span>"Scopri la collezione"</span>
                                <span
                                    aria-hidden="true"
                                    class="block h-px w-12 bg-ivory transition-[width] duration-700 ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:w-24"
                                ></span>
                            </a>
                        </div>

                        <figure class="lg:col-span-5 fade-in" style="animation-delay: 550ms">
                            <img
                                src="/products/hero-foils-crossed.png"
                                alt="Due fogli da scherma incrociati: cordino blu su pomello in ottone scanalato, su sfondo ardesia."
                                width="1122"
                                height="1402"
                                class="w-full h-auto"
                            />
                            <figcaption class="tracker text-stone mt-6 flex justify-between gap-4">
                                <span>"Foglio Notturno · MMXXVI"</span>
                                <span>"Ref. GO·01"</span>
                            </figcaption>
                        </figure>
                    </div>
                </section>

                <section id="atelier" class="bg-cream text-ink">
                    <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                        <p class="tracker text-oxblood mb-20 text-center">
                            "I \u{00a0}·\u{00a0} L'Atelier"
                        </p>

                        <div class="grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-20 items-end">
                            <figure class="lg:col-span-6">
                                <img
                                    src="/products/foil-noir.png"
                                    alt="Foglio con impugnatura nera e pomello in ottone, posato su marmo color crema."
                                    width="1254"
                                    height="1254"
                                    class="w-full h-auto"
                                />
                            </figure>

                            <div class="lg:col-span-6 lg:pb-8 flex flex-col gap-10">
                                <h2 class="font-display font-light text-[clamp(2.25rem,4.5vw,4.5rem)] leading-[1.05] tracking-[-0.01em]">
                                    "Quattro generazioni"
                                    <br/>
                                    <span class="italic">"nella stessa bottega."</span>
                                </h2>

                                <p class="font-serif text-[1.125rem] leading-[1.65] text-graphite max-w-[58ch] first-letter:font-display first-letter:text-[5.5rem] first-letter:float-left first-letter:leading-[0.85] first-letter:mr-3 first-letter:mt-1 first-letter:text-oxblood">
                                    "Da centoventotto anni la famiglia Omega forgia armi sportive nella bottega di Via Mazzini. Le lame sono temprate a 880 °C e ribattute a mano. Il pomello, al tornio. Il cordino, intrecciato e cucito uno per uno. Nulla di automatizzato. Ogni foglio porta inciso il numero del maestro che l'ha forgiato e l'anno della sua nascita."
                                </p>

                                <dl class="grid grid-cols-1 sm:grid-cols-3 gap-8 pt-12 border-t border-ink/15">
                                    <div class="flex flex-col gap-2">
                                        <dt class="tracker text-graphite/70">"Fondazione"</dt>
                                        <dd class="font-display text-3xl">"MDCCCXCVII"</dd>
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <dt class="tracker text-graphite/70">"Maestri"</dt>
                                        <dd class="font-display text-3xl">"IV gen."</dd>
                                    </div>
                                    <div class="flex flex-col gap-2">
                                        <dt class="tracker text-graphite/70">"Lame · anno"</dt>
                                        <dd class="font-display text-3xl">"CXL"</dd>
                                    </div>
                                </dl>
                            </div>
                        </div>
                    </div>
                </section>

                <section id="calibro" class="bg-ink">
                    <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                        <p class="tracker text-stone mb-20 text-center">
                            "II \u{00a0}·\u{00a0} Calibro GO·01"
                        </p>

                        <div class="grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-20 items-center">
                            <div class="lg:col-span-5 flex flex-col gap-10 lg:order-2">
                                <h2 class="font-display font-light text-[clamp(2.25rem,4.5vw,4.25rem)] leading-[1.05] tracking-[-0.01em]">
                                    "Foglio Italiano"
                                    <br/>
                                    <span class="italic">"da pedana."</span>
                                </h2>
                                <p class="font-serif text-[1.125rem] leading-[1.65] text-stone max-w-[44ch]">
                                    "Lama maraging temprata in olio, guardia in acciaio satinato, cordino in cotone egiziano teso a mano. Bilanciamento italiano, vicino alla guardia. Omologata FIE."
                                </p>
                            </div>

                            <figure class="lg:col-span-7 lg:order-1">
                                <img
                                    src="/products/foil-laguna.png"
                                    alt="Foglio con impugnatura verde laguna su superficie chiara."
                                    width="1254"
                                    height="1254"
                                    class="w-full h-auto"
                                />
                            </figure>
                        </div>

                        <dl class="mt-24 lg:mt-32 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 border-t border-rule">
                            <Spec label="Lunghezza totale" value="110 cm"/>
                            <Spec label="Peso" value="480 g"/>
                            <Spec label="Punto d'equilibrio" value="17,5 cm dalla guardia"/>
                            <Spec label="Lama" value="Acciaio maraging, FIE omologata"/>
                            <Spec label="Impugnatura" value="Cordino italiano, sei colori"/>
                            <Spec label="Pomello" value="Ottone tornito a mano"/>
                        </dl>
                    </div>
                </section>

                <section id="collezione" class="bg-cream text-ink">
                    <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                        <div class="flex flex-col items-center gap-6 mb-24">
                            <p class="tracker text-oxblood">"III \u{00a0}·\u{00a0} La Collezione"</p>
                            <h2 class="font-display font-light text-[clamp(2.5rem,5vw,5rem)] leading-[1.05] tracking-[-0.01em] text-center">
                                "Tre figure."
                                <br/>
                                <span class="italic">"Tutte fatte a Verona."</span>
                            </h2>
                        </div>

                        <div class="grid grid-cols-1 md:grid-cols-3 gap-x-10 gap-y-20">
                            <ProductCard
                                name="Foglio Notturno"
                                subtitle="Cordino nero · pomello in ottone"
                                src="/products/foil-noir.png"
                            />
                            <ProductCard
                                name="Foglio Laguna"
                                subtitle="Seta verde acqua · pomello satinato"
                                src="/products/foil-laguna.png"
                            />
                            <ProductCard
                                name="Impugnature in cordino"
                                subtitle="Sei colori · filato di cotone egiziano"
                                src="/products/impugnature-cordino.png"
                            />
                        </div>
                    </div>
                </section>

                <footer class="bg-ink border-t border-rule">
                    <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-24 lg:py-32">
                        <div class="grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-16 mb-24">
                            <div class="lg:col-span-4 flex flex-col gap-6">
                                <p class="font-display text-3xl tracking-[0.18em] text-ivory">
                                    "GRAND\u{00a0}OMEGA"
                                </p>
                                <p class="font-serif text-stone leading-[1.6] max-w-[28ch]">
                                    "Via Mazzini 14 · 37121 Verona · Italia"
                                    <br/>
                                    "Lun–Ven 9–18 · Sabato su appuntamento"
                                </p>
                            </div>

                            <div class="lg:col-span-5 flex flex-col gap-6">
                                <p class="tracker text-stone">"Lettera dell'atelier"</p>
                                <p class="font-serif text-ivory text-lg max-w-[36ch]">
                                    "Quattro lettere l'anno, scritte dall'atelier. Niente sconti, niente urgenza."
                                </p>
                                <form class="flex flex-col sm:flex-row sm:items-end gap-4 mt-2">
                                    <label class="flex-1 flex flex-col gap-2">
                                        <span class="tracker text-stone">"Indirizzo email"</span>
                                        <input
                                            type="email"
                                            required
                                            class="bg-transparent border-b border-rule focus:border-ivory outline-none py-2 text-ivory font-serif text-lg transition-colors duration-500"
                                        />
                                    </label>
                                    <button
                                        type="submit"
                                        class="tracker text-ivory pb-3 hover:text-oxblood transition-colors duration-500 text-left"
                                    >
                                        "Iscriviti →"
                                    </button>
                                </form>
                            </div>

                            <nav class="lg:col-span-3 lg:justify-self-end flex flex-col gap-3 tracker text-stone">
                                <a href="#atelier" class="hover:text-ivory transition-colors duration-500">"L'Atelier"</a>
                                <a href="#calibro" class="hover:text-ivory transition-colors duration-500">"Il Calibro"</a>
                                <a href="#collezione" class="hover:text-ivory transition-colors duration-500">"La Collezione"</a>
                                <a href="#" class="hover:text-ivory transition-colors duration-500">"Storia"</a>
                                <a href="#" class="hover:text-ivory transition-colors duration-500">"Contatti"</a>
                                <a href="#" class="hover:text-ivory transition-colors duration-500">"Stampa"</a>
                            </nav>
                        </div>

                        <div class="pt-10 border-t border-rule flex flex-col sm:flex-row justify-between gap-4 tracker text-stone">
                            <p>"© MMXXVI Grand Omega Srl · P.IVA IT 01234567890"</p>
                            <p>"Maître d'armes dal MDCCCXCVII"</p>
                        </div>
                    </div>
                </footer>
            </main>
        </>
    }
}

#[component]
fn Spec(label: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-8 px-2 border-b border-rule sm:[&:nth-child(2n)]:border-l lg:[&:nth-child(2n)]:border-l-0 lg:[&:not(:nth-child(3n+1))]:border-l border-rule">
            <dt class="tracker text-stone">{label}</dt>
            <dd class="font-display text-2xl text-ivory">{value}</dd>
        </div>
    }
}

#[component]
fn ProductCard(name: &'static str, subtitle: &'static str, src: &'static str) -> impl IntoView {
    view! {
        <article class="group flex flex-col gap-6">
            <div class="overflow-hidden bg-ivory">
                <img
                    src=src
                    alt=name
                    width="1254"
                    height="1254"
                    class="w-full h-auto transition-transform duration-[1400ms] ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:scale-[1.02]"
                />
            </div>
            <div class="flex flex-col gap-2 pt-2">
                <h3 class="font-display text-2xl italic">{name}</h3>
                <p class="font-serif text-graphite/80 text-base">{subtitle}</p>
                <a href="#" class="tracker text-ink mt-4 inline-flex items-center gap-3 w-fit">
                    <span>"Su richiesta"</span>
                    <span
                        aria-hidden="true"
                        class="block h-px w-8 bg-ink transition-[width] duration-700 ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:w-16"
                    ></span>
                </a>
            </div>
        </article>
    }
}
