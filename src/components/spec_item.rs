use crate::content::Spec;
use leptos::prelude::*;

#[component]
pub fn SpecItem(spec: Spec) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 py-8 px-2 border-b border-rule sm:[&:nth-child(2n)]:border-l lg:[&:nth-child(2n)]:border-l-0 lg:[&:not(:nth-child(3n+1))]:border-l border-rule">
            <dt class="tracker text-stone">{spec.label}</dt>
            <dd class="font-display text-2xl text-ivory">{spec.value}</dd>
        </div>
    }
}
