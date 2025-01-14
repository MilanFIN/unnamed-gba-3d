pub const LEVELSIZE: usize = 30;

const LEVEL1: &str = r#"
[
    { "type": "cube", "data": { "size": 3, "x": 0, "y": 0, "z": 0 } }
]
"#;

const LEVEL2: &str = r#"
[
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -5, "z": 0, "color": 2 } },
    { "type": "crumbling", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": 3, "rotation": -0.1, "lifetime": 1 } },
    {
        "type": "mover",
        "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -5,
                "pos_a_x": 0, "pos_a_y": -3.5, "pos_a_z": -5,
                "pos_b_x": 10, "pos_b_y": -3.5, "pos_b_z": -5,
                "speed": 1, "wait": 20
                }
    },
    { "type": "finish", "data": {"size": 2, "x": 0, "y": -5, "z": 3, "color": 2 } },
    { "type": "switch", "data": {"size": 2, "x": 0, "y": -3.5, "z": 1, "color": 3, "rotation": 0.0 } },
    { "type": "wireframe", "data": {"xsize": 5, "ysize": 1, "zsize": 5, "x": 3, "y": -5, "z": 3, "color": 2 } }

] 
"#;

/*
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -5 } }
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0, "y": -3.5, "z": -10 } }
*/

//    { "type": "cube", "data": { "size": 2.0, "x": 10, "y": -10, "z":0  } },
//    { "type": "rectangle", "data": { "xsize": 5, "ysize": 0.2, "zsize": 5, "x": 0, "y": -10, "z": 0 } }
//    { "type": "cube", "data": { "size": 1.0, "x": 4.2, "y": 0, "z": 0 } }

// The result is a constant array of string slices.
pub const LEVELS: [&'static str; 2] = [LEVEL1, LEVEL2];
