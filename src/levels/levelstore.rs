// DO NOT EDIT: This file was automatically generated by bundle_levels.py.
// Any modifications will be overwritten during the next build.



pub const LEVELSIZE: usize = 15;

const LEVEL1: &str = r#"[
    { "type": "cube", "data": { "size": 3, "x": 10, "y": 0, "z": 0 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 5, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "ice", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 3, "y": 1, "z": 0, "color": 2, "acceleration": 0.004, "rotation": 0.0 } },
    { "type": "bounce", "data": { "size": 2, "height": 1, "x": 0, "y": 1, "z": 4, "color": 4, "power": 1.5, "rotation": 0.0 } }
]
"#;

const LEVEL2: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 0.2,
            "ysize": 1,
            "zsize": 5,
            "x": 0,
            "y": -5,
            "z": 0,
            "rotation": 0.125,
            "color": 2
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 0.2,
            "ysize": 1,
            "zsize": 5,
            "x": 1,
            "y": -4,
            "z": 0,
            "rotation": 0.125,
            "color": 2
        }
    }
]
"#;

const LEVEL3: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -0.65, "y": 0, "z": 5, "color": 2, "rotation": -0.04 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -2.56, "y": 0, "z": 9.5, "color": 2, "rotation": -0.08 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -5.6, "y": 0, "z": 13.5, "color": 2, "rotation": -0.12 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -9.5, "y": 0, "z": 16.5, "color": 2, "rotation": -0.16 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -14.5, "y": 0, "z": 18.5, "color": 2, "rotation": -0.18 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -19, "y": 0, "z": 19, "color": 2, "rotation": -0.22 } },
    { "type": "finish", "data": {"size": 3, "x": -19, "y": 3, "z": 19, "color": 2, "rotation": -0.22 } }
]"#;

const LEVEL4: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 4.0, "y": 3.5, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0.0, "y": 7, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 4.0, "y": 10.5, "z": 0, "color": 2, "rotation": 0.0 } },
    {"type": "mover",
        "data": { 
            "xsize": 4, "ysize": 1, "zsize": 4, "x": 8, "y": 10.5, "z": 0,
            "pos_a_x": 8, "pos_a_y": 10.5, "pos_a_z": 0,
            "pos_b_x": 28, "pos_b_y": 15, "pos_b_z": 7,
            "speed": 2, "wait": 20
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 32, "y": 15, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 28, "y": 18.5, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 32, "y": 22, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 28, "y": 25.5, "z": 7, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 28, "y": 28.5, "z": 7, "color": 2, "rotation": -0.22 } }

]"#;

const LEVEL5: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 2, "x": 0.0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 5, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 1, "y": 0, "z": 10, "rotation": 0.1, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 4, "y": 0, "z": 15, "rotation": 0.2, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 3, "x": 10, "y": 0, "z": 17, "color": 2, "rotation": 0.3 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 16, "y": 0, "z": 15, "rotation": 0.4, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 18, "y": 0, "z": 20, "rotation": 0.2, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 21, "y": 0, "z": 25, "rotation": 0.1, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 5, "ysize": 1, "zsize": 3, "x": 21, "y": 0, "z": 30, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 4, "ysize": 0.5, "zsize": 3, "x": 21, "y": 1, "z": 35, "rotation": 0.0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 4, "ysize": 0.5, "zsize": 3, "x": 21, "y": 2, "z": 40, "rotation": 0.0, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 4, "ysize": 1, "zsize": 3, "x": 21, "y": 4, "z": 45, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 21, "y": 7, "z": 45, "color": 2, "rotation": 0 } }

]"#;

const LEVEL6: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 5, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 10, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 15, "rotation": 0, "lifetime": 25 } },
    { "type": "crumbling", "data": { "xsize": 3, "ysize": 0.5, "zsize": 3, "x": 0, "y": 0, "z": 20, "rotation": 0, "lifetime": 25 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
    { "type": "switch", "data": {"size": 2, "x": 0, "y": 1, "z": 25.5, "color": 3, "rotation": 0.125 } },
    { "type": "wireframe", "data": {"xsize": 2, "ysize": 1, "zsize": 2, "x": -5, "y": 0, "z": 0, "color": 0 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": -10, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": -10, "y": 3, "z": 0, "color": 2, "rotation": 0.25 } }
]"#;

const LEVEL7: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 3, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 6, "color": 2, "rotation": 0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 9, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 12, "color": 2, "rotation": -0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 15, "color": 2, "rotation": 0.0 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 1, "y": 0, "z": 18, "color": 2, "rotation": 0.1 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 21, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 24, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 27, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 30, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 0.1, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 33, "color": 2, "rotation": 0.25 } },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 2, "y": 0, "z": 38, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 2, "y": 3, "z": 38, "color": 2, "rotation": 0 } }
]"#;

const LEVEL8: &str = r#"[
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 0, "color": 2, "rotation": 0.0 } },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 5,
            "pos_a_x": -5, "pos_a_y": 0, "pos_a_z": 5,
            "pos_b_x": 5, "pos_b_y": 0, "pos_b_z": 5,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 10,
            "pos_a_x": 5, "pos_a_y": 0, "pos_a_z": 10,
            "pos_b_x": -5, "pos_b_y": 0, "pos_b_z": 10,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 15,
            "pos_a_x": -5, "pos_a_y": 0, "pos_a_z": 15,
            "pos_b_x": 5, "pos_b_y": 0, "pos_b_z": 15,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 20,
            "pos_a_x": 5, "pos_a_y": 0, "pos_a_z": 20,
            "pos_b_x": -5, "pos_b_y": 0, "pos_b_z": 20,
            "speed": 0.7, "wait": 3
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
       {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 35,
            "pos_a_x": 0, "pos_a_y": 0, "pos_a_z": 30,
            "pos_b_x": 0, "pos_b_y": 0, "pos_b_z": 40,
            "speed": 0.7, "wait": 5
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 25, "color": 2, "rotation": 0.0 } },
       {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 50,
            "pos_a_x": 0, "pos_a_y": 0, "pos_a_z": 55,
            "pos_b_x": 0, "pos_b_y": 0, "pos_b_z": 45,
            "speed": 0.7, "wait": 5
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 60, "color": 2, "rotation": 0.0 } },

    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 65,
            "pos_a_x": 0, "pos_a_y": 10, "pos_a_z": 65,
            "pos_b_x": 0, "pos_b_y": -10, "pos_b_z": 65,
            "speed": 0.7, "wait": 3
        }
    },
    {"type": "mover",
        "data": { 
            "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 70,
            "pos_a_x": 0, "pos_a_y": -10, "pos_a_z": 70,
            "pos_b_x": 0, "pos_b_y": 10, "pos_b_z": 70,
            "speed": 0.7, "wait": 3
        }
    },
    { "type": "rectangle", "data": { "xsize": 3, "ysize": 1, "zsize": 3, "x": 0, "y": 0, "z": 75, "color": 2, "rotation": 0.0 } },
    { "type": "finish", "data": {"size": 3, "x": 0, "y": 3, "z": 75, "color": 2, "rotation": 0 } }
]"#;

const LEVEL9: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 0,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 6,
            "color": 5
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 12,
            "color": 2
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 3,
            "y": 0,
            "z": 17,
            "color": 5
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": -3,
            "y": 0,
            "z": 22,
            "color": 5
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 3,
            "y": 0,
            "z": 27,
            "color": 5
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 32,
            "color": 2
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 2,
            "x": 5,
            "y": 3,
            "z": 32,
            "color": 5,
            "rotation": 0.25
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 2,
            "x": 10,
            "y": 6,
            "z": 32,
            "color": 5,
            "rotation": 0.25
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 15.5,
            "y": 9,
            "z": 32,
            "color": 2,
            "rotation": 0.25
        }
    },
    {
        "type": "finish",
        "data": {
            "size": 3,
            "x": 15.5,
            "y": 12,
            "z": 32,
            "color": 2,
            "rotation": 0.25
        }
    }
]
"#;

const LEVEL10: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 0,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 3,
            "z": 6,
            "color": 5
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 0,
            "y": 6,
            "z": 12,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 5,
            "y": 9,
            "z": 17,
            "color": 5,
            "rotation": 0.125
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 8,
            "y": 12,
            "z": 20,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
    {
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 14,
            "y": 15,
            "z": 22,
            "color": 5,
            "rotation": 0.25
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 19,
            "y": 18,
            "z": 22,
            "color": 2,
            "rotation": 0.25
        }
    },
    {
        "type": "finish",
        "data": {
            "size": 3,
            "x": 19,
            "y": 21,
            "z": 22,
            "color": 2,
            "rotation": 0.25
        }
    }
]
"#;

const LEVEL11: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 0,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 0,
            "y": 2,
            "z": 5,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 4,
            "z": 10,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 5,
            "y": 6,
            "z": 10,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 9,
            "y": 10,
            "z": 14,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 13,
            "y": 14,
            "z": 10,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 2,
            "ysize": 1,
            "zsize": 2,
            "x": 19,
            "y": 18,
            "z": 9,
            "rotation": 0.125,
            "lifetime": 10,
            "color": 2

        }
    },
    {
        "type": "crumbling",
        "data": {
            "xsize": 2,
            "ysize": 1,
            "zsize": 2,
            "x": 23,
            "y": 18,
            "z": 13,
            "rotation": 0.125,
            "lifetime": 10,
            "color": 0
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 2,
            "ysize": 1,
            "zsize": 2,
            "x": 27,
            "y": 18,
            "z": 17,
            "rotation": 0.125,
            "lifetime": 10,
            "color": 2
        }
    },
    {
        "type": "finish",
        "data": {
            "size": 3,
            "x": 27,
            "y": 21,
            "z": 17,
            "color": 2,
            "rotation": 0.125
        }
    }
]
"#;

const LEVEL12: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 0,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "mover",
        "data": {
            "xsize": 4,
            "ysize": 1,
            "zsize": 4,
            "x": 0,
            "y": 0,
            "z": 5,
            "pos_a_x": 0,
            "pos_a_y": 0,
            "pos_a_z": 5,
            "pos_b_x": 0,
            "pos_b_y": 0,
            "pos_b_z": 10,
            "speed": 0.5,
            "wait": 20
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 15,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 18,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 3,
            "z": 23,
            "color": 2,
            "rotation": 0.0
        }
    },
	{
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 4,
            "y": 4,
            "z": 23,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 10,
            "y": 8,
            "z": 23,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 16,
            "y": 8.5,
            "z": 23,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 22,
            "y": 9,
            "z": 23,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "ice",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 28,
            "y": 9.5,
            "z": 23,
            "color": 5,
            "rotation": 0.0
        }
    },
	{
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 34,
            "y": 10,
            "z": 23,
            "color": 2,
            "rotation": 0.0
        }
    },
	{
        "type": "bounce",
        "data": {
            "size": 4,
            "height": 1,
            "x": 38,
            "y": 10,
            "z": 27,
            "color": 4,
            "power": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 40,
            "y": 20,
            "z": 30,
            "color": 2,
            "rotation": 0
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 46,
            "y": 20,
            "z": 30,
            "color": 2,
            "rotation": 0
        }
    },
    {
        "type": "finish",
        "data": {
            "size": 3,
            "x": 46,
            "y": 23,
            "z": 30,
            "color": 2,
            "rotation": 0.25
        }
    }
]
"#;

const LEVEL13: &str = r#"[
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 0,
            "color": 2,
            "rotation": 0.0
        }
    },

    {
        "type": "wireframe",
        "data": {
            "xsize": 2,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 0,
            "z": 6,
            "color": 0,
            "solid": true,
            "rotation": 0.125
        }
    },

    {
        "type": "bounce",
        "data": {
            "size": 2,
            "height": 1,
            "x": 4.75,
            "y": 0,
            "z": 9.5,
            "color": 4,
            "power": 1.5,
            "rotation": 0.0
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 3,
            "y": 4,
            "z": 14,
            "color": 2,
            "rotation": 0.0
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 5,
            "z": 18,
            "color": 2,
            "rotation": 0.125
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 6,
            "y": 6,
            "z": 22,
            "color": 2,
            "rotation": 0.125
        }
    },
    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 0,
            "y": 7,
            "z": 25,
            "color": 2,
            "rotation": 0.125
        }
    },
    {
        "type": "switch",
        "data": {
            "size": 2,
            "x": 0,
            "y": 8,
            "z": 25,
            "color": 3,
            "rotation": 0.25
        }
    },

    {
        "type": "wireframe",
        "data": {
            "xsize": 2,
            "ysize": 1,
            "zsize": 2,
            "x": 10,
            "y": 0,
            "z": 9.5,
            "color": 0,
            "solid": false,
            "rotation": 0.0
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 15,
            "y": 0,
            "z": 9.5,
            "color": 2,
            "rotation": 0.0
        }
    },

    {
        "type": "mover",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 20,
            "y": 0,
            "z": 9.5,
            "pos_a_x": 20,
            "pos_a_y": 0,
            "pos_a_z": 9.5,
            "pos_b_x": 20,
            "pos_b_y": 10,
            "pos_b_z": 9.5,
            "speed": 0.7,
            "wait": 15
        }
    },

    {
        "type": "rectangle",
        "data": {
            "xsize": 3,
            "ysize": 1,
            "zsize": 3,
            "x": 25,
            "y": 10,
            "z": 9.5,
            "color": 2,
            "rotation": 0.0
        }
    },
    {
        "type": "finish",
        "data": {
            "size": 3,
            "x": 25,
            "y": 13,
            "z": 9.5,
            "color": 2,
            "rotation": 0.25
        }
    }
]
"#;

pub const LEVELS: [&'static str; 13] = [LEVEL1, LEVEL2, LEVEL3, LEVEL4, LEVEL5, LEVEL6, LEVEL7, LEVEL8, LEVEL9, LEVEL10, LEVEL11, LEVEL12, LEVEL13];