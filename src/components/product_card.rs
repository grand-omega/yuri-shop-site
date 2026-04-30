use crate::content::Product;
use leptos::prelude::*;

#[component]
pub fn ProductCard(product: Product) -> impl IntoView {
    view! {
        <article class="group flex flex-col gap-6">
            <div class="overflow-hidden bg-ivory">
                <img
                    src=product.image
                    alt=product.name
                    width="1254"
                    height="1254"
                    class="w-full h-auto transition-transform duration-[1400ms] ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:scale-[1.02]"
                />
            </div>
            <div class="flex flex-col gap-2 pt-2">
                <h3 class="font-display text-2xl italic">{product.name}</h3>
                <p class="font-serif text-graphite/80 text-base">{product.subtitle}</p>
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
