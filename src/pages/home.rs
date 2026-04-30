use crate::api::catalog::list_products;
use crate::components::{Footer, Header, ProductCard};
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Header/>
        <main class="pt-[122px] lg:pt-[81px] flex-1">
            <HeroSection/>
            <AtelierSection/>
            <CalibroSection/>
            <CollectionSection/>
            <Footer/>
        </main>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
        <section class="relative bg-ink">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 pt-20 pb-28 lg:py-32 grid grid-cols-1 lg:grid-cols-12 gap-y-20 lg:gap-x-16 items-center min-h-[calc(100vh-122px)] lg:min-h-[calc(100vh-81px)]">
                <div class="lg:col-span-7 flex flex-col gap-10 lg:gap-12">
                    <p class="tracker text-stone rise" style="animation-delay: 150ms">
                        "Verona · MMXXVI · Atelier since MDCCCXCVII"
                    </p>

                    <h1
                        class="font-display font-light text-[clamp(3rem,7.5vw,8.25rem)] leading-[0.95] tracking-[-0.012em] text-ivory rise"
                        style="animation-delay: 350ms"
                    >
                        "The art of"
                        <br/>
                        <span class="italic font-normal">"fencing,"</span>
                        <br/>
                        "hand-forged."
                    </h1>

                    <div class="h-px w-28 bg-oxblood origin-left draw-x" style="animation-delay: 950ms"></div>

                    <p
                        class="font-serif text-[1.25rem] leading-[1.55] text-stone max-w-[44ch] rise"
                        style="animation-delay: 650ms"
                    >
                        "Maîtres d'armes since 1897. Each blade carries four generations of artisanship from our atelier on Via Mazzini."
                    </p>

                    <A
                        href="/collezione"
                        attr:class="group inline-flex items-center gap-4 tracker text-ivory rise w-fit"
                        attr:style="animation-delay: 1050ms"
                    >
                        <span>"Discover the collection"</span>
                        <span
                            aria-hidden="true"
                            class="block h-px w-12 bg-ivory transition-[width] duration-700 ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:w-24"
                        ></span>
                    </A>
                </div>

                <figure class="lg:col-span-5 fade-in" style="animation-delay: 550ms">
                    <img
                        src="/products/hero-foils-crossed.png"
                        alt="Two fencing foils crossed: blue cord over a fluted brass pommel, on a slate background."
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
    }
}

#[component]
fn AtelierSection() -> impl IntoView {
    view! {
        <section id="atelier" class="bg-cream text-ink">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                <p class="tracker text-oxblood mb-20 text-center">
                    "I \u{00a0}·\u{00a0} L'Atelier"
                </p>

                <div class="grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-20 items-end">
                    <figure class="lg:col-span-6">
                        <img
                            src="/products/foil-noir.png"
                            alt="A foil with a black cord grip and a brass pommel, resting on cream-coloured marble."
                            width="1254"
                            height="1254"
                            class="w-full h-auto"
                        />
                    </figure>

                    <div class="lg:col-span-6 lg:pb-8 flex flex-col gap-10">
                        <h2 class="font-display font-light text-[clamp(2.25rem,4.5vw,4.5rem)] leading-[1.05] tracking-[-0.01em]">
                            "Four generations"
                            <br/>
                            <span class="italic">"in the same workshop."</span>
                        </h2>

                        <p class="font-serif text-[1.125rem] leading-[1.65] text-graphite max-w-[58ch] first-letter:font-display first-letter:text-[5.5rem] first-letter:float-left first-letter:leading-[0.85] first-letter:mr-3 first-letter:mt-1 first-letter:text-oxblood">
                            "For one hundred and twenty-eight years the Omega family has forged sport arms in the workshop on Via Mazzini. Blades are tempered at 880 °C and re-struck by hand. The pommel is turned on the lathe. The cord grip is woven and stitched one at a time. Nothing is automated. Every foil is engraved with the master's number and his year of birth."
                        </p>

                        <dl class="grid grid-cols-1 sm:grid-cols-3 gap-8 pt-12 border-t border-ink/15">
                            <div class="flex flex-col gap-2">
                                <dt class="tracker text-graphite/70">"Founded"</dt>
                                <dd class="font-display text-3xl">"MDCCCXCVII"</dd>
                            </div>
                            <div class="flex flex-col gap-2">
                                <dt class="tracker text-graphite/70">"Masters"</dt>
                                <dd class="font-display text-3xl">"IV gen."</dd>
                            </div>
                            <div class="flex flex-col gap-2">
                                <dt class="tracker text-graphite/70">"Blades · year"</dt>
                                <dd class="font-display text-3xl">"CXL"</dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CalibroSection() -> impl IntoView {
    view! {
        <section id="calibro" class="bg-ink">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                <p class="tracker text-stone mb-20 text-center">
                    "II \u{00a0}·\u{00a0} Il Calibro GO·01"
                </p>

                <div class="grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-20 items-center">
                    <div class="lg:col-span-5 flex flex-col gap-10 lg:order-2">
                        <h2 class="font-display font-light text-[clamp(2.25rem,4.5vw,4.25rem)] leading-[1.05] tracking-[-0.01em]">
                            "The Italian"
                            <br/>
                            <span class="italic">"competition foil."</span>
                        </h2>
                        <p class="font-serif text-[1.125rem] leading-[1.65] text-stone max-w-[44ch]">
                            "Maraging steel blade tempered in oil, satin stainless guard, Egyptian cotton cord drawn by hand. Italian balance, close to the guard. FIE-homologated."
                        </p>
                        <A
                            href="/collezione/foglio-notturno"
                            attr:class="group tracker text-ivory inline-flex items-center gap-3 w-fit"
                        >
                            <span>"View specifications"</span>
                            <span
                                aria-hidden="true"
                                class="block h-px w-10 bg-ivory transition-[width] duration-700 ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:w-20"
                            ></span>
                        </A>
                    </div>

                    <figure class="lg:col-span-7 lg:order-1">
                        <img
                            src="/products/foil-laguna.png"
                            alt="A foil with a lagoon-green grip on a pale surface."
                            width="1254"
                            height="1254"
                            class="w-full h-auto"
                        />
                    </figure>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CollectionSection() -> impl IntoView {
    let products = Resource::new(|| (), |_| async move { list_products().await });

    view! {
        <section id="collezione" class="bg-cream text-ink">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                <div class="flex flex-col items-center gap-6 mb-24">
                    <p class="tracker text-oxblood">"III \u{00a0}·\u{00a0} La Collezione"</p>
                    <h2 class="font-display font-light text-[clamp(2.5rem,5vw,5rem)] leading-[1.05] tracking-[-0.01em] text-center">
                        "Three forms."
                        <br/>
                        <span class="italic">"All made in Verona."</span>
                    </h2>
                </div>

                <Suspense fallback=move || view! {
                    <p class="tracker text-graphite/60 text-center">"Loading…"</p>
                }>
                    {move || products.get().map(|res| match res {
                        Ok(items) => view! {
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-x-10 gap-y-20">
                                {items.into_iter().map(|p| view! {
                                    <ProductCard product=p/>
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <p class="tracker text-oxblood text-center">{e.to_string()}</p>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </section>
    }
}
