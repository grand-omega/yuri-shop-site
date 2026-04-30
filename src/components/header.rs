use crate::content::{PRIMARY_NAV, UTILITY_NAV};
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="fixed top-0 inset-x-0 z-50 bg-ink/92 backdrop-blur-md border-b border-rule">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-5 grid grid-cols-1 gap-5 lg:grid-cols-3 lg:gap-0 items-center">
                <nav class="flex gap-5 md:gap-7 tracker text-stone justify-center lg:justify-start">
                    <For
                        each=|| PRIMARY_NAV
                        key=|link| link.href
                        children=|link| view! {
                            <a href=link.href class="hover:text-ivory transition-colors duration-500">
                                {link.label}
                            </a>
                        }
                    />
                </nav>
                <a href="/" class="font-display text-[1.4rem] md:text-[1.65rem] tracking-[0.22em] text-center text-ivory leading-none">
                    "GRAND\u{00a0}OMEGA"
                </a>
                <nav class="hidden lg:flex gap-7 tracker text-stone justify-end">
                    <For
                        each=|| UTILITY_NAV
                        key=|link| link.label
                        children=|link| view! {
                            <a href=link.href class="hover:text-ivory transition-colors duration-500">
                                {link.label}
                            </a>
                        }
                    />
                </nav>
            </div>
        </header>
    }
}
