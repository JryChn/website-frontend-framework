pub fn wireLaneWithSvg(svg_path: &str, id:&str) ->String{
    return
     r#"
    var Engine = Matter.Engine,
        Render = Matter.Render,
        Runner = Matter.Runner,
        Composite = Matter.Composite,
        Composites = Matter.Composites,
        Constraint = Matter.Constraint,
        MouseConstraint = Matter.MouseConstraint,
        Mouse = Matter.Mouse,
        Bodies = Matter.Bodies,
        Body = Matter.Body;
        Events = Matter.Events;
    // create an engine
    var engine = Engine.create();
    var world = engine.world;

    // create a renderer
    var render = Render.create({
        element: document.getElementById(""#.to_owned() +id+ &*r#""),
        engine: engine,
        options:{
            width: 700,
            height:300,
            wireframes: false,
            background: "transparent",
        }
    });
    // create two boxes and a ground
    var boxA =Composites.stack(370,-20,1,15,0,0,function(x,y){
        return Bodies.rectangle(x+y*1.1, -20, 3, 7,{ render:{fillStyle:"black"}, collisionFilter: { group: -10 } });
    });
    var head = Bodies.circle(150, 0, 17, {
        render: {
            sprite: {
                texture: '"#.to_owned() +svg_path+r##"'
            }
        }}
    );
    Events.on(engine,"afterUpdate",(_e)=>{
        var yellow_light = document.getElementById("yellow_light");
        if (yellow_light != null){
            yellow_light.style.top = (head.position.x *0.3).toString()+"px";
            yellow_light.style.left = (head.position.y *2.3 ).toString()+"px";
        }
    });
    Composite.add(boxA,head);
    Composites.chain(boxA, 0, 0.5, 0, -0.5, { stiffness: 1, length: 0, render: { type: 'line',strokeStyle: 'black' , lineWidth:3} });
    Composite.add(boxA, Constraint.create({
        bodyB: boxA.bodies[0],
        pointB: { x: 0, y: 0 },
        pointA: { x: boxA.bodies[0].position.x, y: boxA.bodies[0].position.y },
        stiffness: 1,
    }));
    // add all of the bodies to the world
    Composite.add(world,[boxA])

    var mouse = Mouse.create(render.canvas),
        mouseConstraint = MouseConstraint.create(engine, {
            mouse: mouse,
            constraint: {
                stiffness: 0.2,
                render: {
                    visible: false
                }
            }
        });
    Events.on(mouseConstraint,"mousemove",()=>{
        var x_range =Math.abs(mouse.position.x - head.position.x);
        var y_range =Math.abs(mouse.position.y - head.position.y);
        if (x_range<17 && y_range <17)
        {
            render.canvas.style.cursor = "pointer";
        }else{
            render.canvas.style.cursor = "default";
        }
    })
    Events.on(mouseConstraint,"mouseup",()=>{
        var x_range =Math.abs(mouse.position.x - head.position.x);
        var y_range =Math.abs(mouse.position.y - head.position.y);
        if (x_range<17 && y_range <17)
        {
            document.documentElement.classList.toggle("dark");
            document.body.classList.toggle("dark:bg-gray-950");
            var yellow_light = document.createElement("div");
            yellow_light.style.position = "absolute";
            yellow_light.style.width = "100px";
            yellow_light.style.height = "100px";
            yellow_light.style.filter = "blur(64px)";
            yellow_light.style.background = "#0533EA";
            yellow_light.style.zIndex = "-50";
            yellow_light.id = "yellow_light";
            if(document.getElementById("yellow_light") == null){
                document.getElementById(""## +id+ &*r#"").appendChild(yellow_light);
            }
        }
    })

    Composite.add(world, mouseConstraint);
    // run the renderer
    Render.run(render);

    // create runner
    var runner = Runner.create();

    // run the engine
    Runner.run(runner, engine);
"#;

}