use crate::components::{Footer, Header, ProductCard, SpecItem};
use crate::content::{CALIBRO_SPECS, PRODUCTS};
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Header/>
            <main class="pt-[122px] lg:pt-[81px] flex-1">
                <HeroSection/>
                <AtelierSection/>
                <CalibroSection/>
                <CollectionSection/>
                <Footer/>
            </main>
        </>
    }
}

#[component]
fn HeroSection() -> impl IntoView {
    view! {
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
    }
}

#[component]
fn CalibroSection() -> impl IntoView {
    view! {
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
                    <For
                        each=|| CALIBRO_SPECS
                        key=|spec| spec.label
                        children=|spec| view! { <SpecItem spec=*spec/> }
                    />
                </dl>
            </div>
        </section>
    }
}

#[component]
fn CollectionSection() -> impl IntoView {
    view! {
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
                    <For
                        each=|| PRODUCTS
                        key=|product| product.name
                        children=|product| view! { <ProductCard product=*product/> }
                    />
                </div>
            </div>
        </section>
    }
}
