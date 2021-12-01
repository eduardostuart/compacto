use std::{fs::File, io::Write};

fn main() {
    // A random JSON from json-generator.com
    let input = r#"
        [
        {
            "_id": "61a6ac85feb4f0140953d432",
            "index": 0,
            "guid": "9ad2b6b6-cce1-442e-80be-c795806625f0",
            "isActive": true,
            "balance": "$3,292.06",
            "picture": "http://placehold.it/32x32",
            "age": 21,
            "eyeColor": "green",
            "name": "Hutchinson Ochoa",
            "gender": "male",
            "company": "ZBOO",
            "email": "hutchinsonochoa@zboo.com",
            "phone": "+1 (828) 470-2742",
            "address": "379 Decatur Street, Chalfant, New Hampshire, 8024",
            "about": "Cupidatat quis incididunt est laborum dolor deserunt enim eu cupidatat dolor ex sint voluptate tempor. Aliqua commodo culpa dolor occaecat irure commodo est consequat. Lorem aliqua labore aute aute tempor magna consequat sunt proident aliqua. Ullamco fugiat esse elit velit ad consequat aute. Ipsum commodo minim ut consequat culpa labore non sint aute sunt proident veniam esse.\r\n",
            "registered": "2020-05-04T02:25:38 -02:00",
            "latitude": -72.17581,
            "longitude": -46.671573,
            "tags": [
            "anim",
            "cillum",
            "elit",
            "aliquip",
            "consectetur",
            "laborum",
            "occaecat"
            ],
            "friends": [
            {
                "id": 0,
                "name": "Lynch Conley"
            },
            {
                "id": 1,
                "name": "Cantu Alford"
            },
            {
                "id": 2,
                "name": "Hopkins Hill"
            }
            ],
            "greeting": "Hello, Hutchinson Ochoa! You have 2 unread messages.",
            "favoriteFruit": "apple"
        }
        ]
    "#;

    let result = compacto::compress_json(input).unwrap();

    let mut output = File::create("compacto/examples/compress-output.json").unwrap();
    output.write_all(result.to_string().as_bytes()).unwrap();

    println!("file generated: compacto/examples/compress-output.json");
}
