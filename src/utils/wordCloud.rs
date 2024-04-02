use std::collections::HashMap;

pub fn word_cloud_maker(keywords: &HashMap<String, i32>) -> String {
    let mut keywords_string = String::new();
    keywords.iter().for_each(|kv| {
        keywords_string.push_str("{name: '");
        keywords_string.push_str(kv.0);
        keywords_string.push_str("',value:");
        keywords_string.push_str(kv.1.to_string().as_str());
        keywords_string.push_str(",emphasis: {textStyle: {color: '#9d2933'}}},");
    });
    let pre_eval = r#"
     var option = {
        tooltip: {},
        series: [ {
            type: 'wordCloud',
            gridSize: 2,
            sizeRange: [12, 150],
            rotationRange: [-90, 90],
            shape: 'cardioid',
            drawOutOfBound: true,
            textStyle: {
            fontFamily: 'outfit',
            fontWeight: 'bold',
                color: function () {
                    return 'rgb(' + [
                        Math.round(Math.random() * 80),
                        Math.round(Math.random() * 20),
                        Math.round(Math.random() * 50)
                    ].join(',') + ')';
                }
            },
            emphasis: {
                textStyle: {
                    shadowBlur: 10,
                    shadowColor: '#333'
                }
            },
            data: [ "#;

    let end_eval = r#"
                    ]
                } ]
             };
                var chart = echarts.init(document.getElementById('article_list_keys'));
                chart.setOption(option);
                window.onresize = chart.resize;
                "#;
    pre_eval.to_owned() + &keywords_string + end_eval
}
