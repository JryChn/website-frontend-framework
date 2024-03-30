use dioxus::prelude::*;
use gloo::console::console_dbg;

#[component]
pub fn Table(date_time: Vec<(String, Vec<(u32, u32)>)>) -> Element {
    let line_color = "border-gray-200";
    let mut date_schedule = Vec::new();
    for (i, (name, ele)) in date_time.iter().enumerate() {
        let mut element_vec = Vec::new();
        for (start, end) in ele {
            element_vec.push(GenerateSchedule(i+1, *start, *end));
        }
        date_schedule.push((name, element_vec));
    }
    let all_events: Vec<Option<VNode>> = date_schedule
        .iter()
        .flat_map(|(_, event)| event)
        .map(|e| e.clone())
        .collect();
    let first_event :Vec<Option<VNode>>= date_schedule.first().unwrap().1.iter().map(|e|e.clone()).collect();
    rsx! {
     div{
         class:"flex-1 -translate-x-12 grid-cols-11 hidden md:grid",
         for e in all_events{
             {e}
         }
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2",
             }
         for i in (0..10){
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2",
                     "{date_schedule.get(i).unwrap().0}"
             }
         }
         for _ in (0..1078){
            div{
                 class:"border-b border-r {line_color}",
                 style:"border-style:dashed solid"
             }
         }
         for _ in (0..11){
            div{
                 class:" border-r border-dashed {line_color}",
             }
         }
     }
             // below is for small screen
     div{
         class:"flex-1 grid -translate-x-12 grid-cols-1 md:hidden",
            for e in first_event{
            {e}
            }
            div{
                 class: "border-r {line_color}",
             }
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2",
                 "{date_schedule.first().unwrap().0}"
             }
         for _ in (0..96){
            div{
                 class:"border-b border-r {line_color}",
                 style:"border-style:dashed solid"
             }
         }
         for _ in (0..2){
            div{
                 class:" border-r border-dashed {line_color}",
             }
         }
     }
    }
}

fn GenerateSchedule(number: usize, start_time: u32, end_time: u32) -> Option<VNode> {
    if number > 10 || number < 1 {
        // error number, must (1..11)
        return rsx! {};
    }
    let col_duration = "grid-column-start: ".to_string()
        + (number + 1).to_string().as_str()
        + ";grid-column-end: "
        + (number + 2).to_string().as_str();
    let row_duration = "grid-row-start: ".to_string()
        + (start_time+2).to_string().as_str()
        + ";grid-row-end: "
        + (end_time+2).to_string().as_str();

    let row_duration_small= "grid-row-start: ".to_string()
        + (start_time+3).to_string().as_str()
        + ";grid-row-end: "
        + (end_time+3).to_string().as_str();
    rsx! {
           div{
                class: "bg-red-900 shadow-[4px_4px_14px_0_rgba(0,0,0,0.25)] rounded-2xl items-center justify-center font-medium text-gray-50 m-2 hover:bg-red-800 hover:shadow-[4px_4px_14px_0_rgba(0,0,0,0.5)] row-star hidden md:flex",
                style: "{col_duration};{row_duration}",
                "busy"
            }
        if number == 1 {
           div{
                class: "bg-red-900 shadow-[4px_4px_14px_0_rgba(0,0,0,0.25)] rounded-2xl flex items-center justify-center font-medium text-gray-50 mx-16 my-2 hover:bg-red-800 hover:shadow-[4px_4px_14px_0_rgba(0,0,0,0.5)] row-star md:hidden",
                style: "grid-column-start: 1;grid-column-end: 2;{row_duration_small}",
                "busy"
            }
        }
    }
}
