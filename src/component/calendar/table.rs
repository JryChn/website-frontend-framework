use dioxus::prelude::*;

#[component]
pub fn Table(date_time: Vec<(String, Vec<(u32, u32)>)>) -> Element {
    let line_color = "border-gray-200";
    let mut date_schedule = Signal::new(Vec::new());
    let mut all_events: Signal<Vec<Option<VNode>>> = Signal::new(Vec::new());
    for (i, (name, ele)) in date_time.iter().enumerate() {
        let mut element_vec = Vec::new();
        for (start, end) in ele {
            element_vec.push(GenerateSchedule(i + 1, *start, *end));
        }
        date_schedule.write().push((name.clone(), element_vec));
    }
    all_events.set(date_schedule.read()
        .iter()
        .flat_map(|(_, event)| event)
        .map(|e| e.clone())
        .collect());
    rsx! {
     div{
         class:"flex-1 -translate-x-12 grid-cols-11 hidden md:grid",
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2 h-8",
             }
         for i in (0..10){
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2 h-8",
                     "{date_schedule.get(i).unwrap().0}"
             }
         }
         for _ in (0..1111){
            div{
                 class:"border-b border-r {line_color} h-8",
                 style:"border-style:dashed solid"
             }
         }
     }
             // below is for small screen
     div{
         class:"flex-1 grid -translate-x-12 grid-cols-1 md:hidden",
            div{
                 class: "border-r {line_color} h-8",
             }
            div{
                 class: "border-r {line_color} text-black font-light text-right pr-2 h-8",
                 "{date_schedule.first().unwrap().0}"
             }
         for _ in (0..100){
            div{
                 class:"border-b border-r {line_color} h-8",
                 style:"border-style:dashed solid"
             }
         }
     }
        // paint events table
        //painting
     div{
         class:"absolute w-1/2 translate-x-1/2 translate-y-16 grid-cols-1 grid-rows-[repeat(96,32px)] grid md:w-3/4 md:translate-x-[15%] md:grid-cols-10",
         for e in all_events(){
             {e}
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
        + number.to_string().as_str()
        + ";grid-column-end: "
        + (number + 1).to_string().as_str();
    let row_duration = "grid-row-start: ".to_string()
        + (start_time + 1).to_string().as_str()
        + ";grid-row-end: "
        + (end_time + 1).to_string().as_str();

    let row_duration_small = "grid-row-start: ".to_string()
        + (start_time + 1).to_string().as_str()
        + ";grid-row-end: "
        + (end_time + 1).to_string().as_str();
    rsx! {
           div{
                class: "bg-red-900 shadow-[4px_4px_14px_0_rgba(0,0,0,0.25)] rounded-2xl items-center justify-center font-medium text-gray-50 m-2 hover:bg-red-800 hover:shadow-[4px_4px_14px_0_rgba(0,0,0,0.5)] hidden md:flex",
                style: "{col_duration};{row_duration}",
                "busy"
            }
        if number == 1 {
           div{
                class: "bg-red-900 shadow-[4px_4px_14px_0_rgba(0,0,0,0.25)] rounded-2xl flex items-center justify-center font-medium text-gray-50 mx-16 my-2 hover:bg-red-800 hover:shadow-[4px_4px_14px_0_rgba(0,0,0,0.5)] md:hidden",
                style: "grid-column-start: 1;grid-column-end: 2;{row_duration_small}",
                "busy"
            }
        }
    }
}
