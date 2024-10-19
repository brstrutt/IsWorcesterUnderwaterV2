use yew::prelude::*;


#[function_component(Waves)]
pub fn waves() -> Html {
    html! {
        <>
            <div
                class="full_canvas background_water"
                color="aqua"
            />
            <div 
                class="full_canvas background_water_2"
                color="aqua"
            />
            <div 
                class="full_canvas clipping_water ShowBelowWaterLevel"
                color="blue"
            />
        </>
    }
}


#[derive(Properties, PartialEq)]
pub struct SolubleTextProps {
    pub shown_above_water: String,
    pub shown_below_water: String
}

#[function_component(SolubleText)]
pub fn soluble_text(SolubleTextProps {shown_above_water, shown_below_water}: &SolubleTextProps) -> Html {
    html! {
        <>
            <div
                class="full_canvas result_wrapper ShowAboveWaterLevel"
            >
                <h1
                    class="result_content"
                    style="color: black;"
                >
                    {shown_above_water}
                </h1>
            </div>
            <div
                class="full_canvas result_wrapper ShowBelowWaterLevel"
            >
                <h1
                    class="result_content" 
                    style="color: white;"
                >
                    {shown_below_water}
                </h1>
            </div>
        </>
    }
}
