use crate::api::catalog::list_products;
use crate::components::{Footer, Header, ProductCard};
use leptos::prelude::*;

#[component]
pub fn CollectionIndex() -> impl IntoView {
    let products = Resource::new(|| (), |_| async move { list_products().await });

    view! {
        <Header/>
        <main class="pt-[122px] lg:pt-[81px] flex-1 bg-cream text-ink">
            <section class="mx-auto max-w-[1480px] px-6 md:px-8 py-32 lg:py-44">
                <div class="flex flex-col items-center gap-6 mb-24">
                    <p class="tracker text-oxblood">"La Collezione"</p>
                    <h1 class="font-display font-light text-[clamp(2.5rem,5vw,5rem)] leading-[1.05] tracking-[-0.01em] text-center">
                        "Three forms."
                        <br/>
                        <span class="italic">"All made in Verona."</span>
                    </h1>
                </div>

                <Suspense fallback=move || view! {
                    <p class="tracker text-graphite/60 text-center">"Loading…"</p>
                }>
                    {move || products.get().map(|res| match res {
                        Ok(items) if items.is_empty() => view! {
                            <p class="tracker text-graphite/60 text-center">
                                "No products published yet."
                            </p>
                        }.into_any(),
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
            </section>
            <Footer/>
        </main>
    }
}
