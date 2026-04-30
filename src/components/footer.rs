use crate::content::FOOTER_NAV;
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
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
                        <For
                            each=|| FOOTER_NAV
                            key=|link| link.label
                            children=|link| view! {
                                <a href=link.href class="hover:text-ivory transition-colors duration-500">
                                    {link.label}
                                </a>
                            }
                        />
                    </nav>
                </div>

                <div class="pt-10 border-t border-rule flex flex-col sm:flex-row justify-between gap-4 tracker text-stone">
                    <p>"© MMXXVI Grand Omega Srl · P.IVA IT 01234567890"</p>
                    <p>"Maître d'armes dal MDCCCXCVII"</p>
                </div>
            </div>
        </footer>
    }
}
