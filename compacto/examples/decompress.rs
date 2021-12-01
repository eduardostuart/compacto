use std::{fs::File, io::Write};

fn main() {
    // A random JSON from json-generator.com
    let input = r#"[[{"0":1,"10":11,"12":13,"14":15,"16":17,"18":[{"19":20,"21":22},{"19":23,"21":24},{"19":25,"21":26}],"2":3,"21":40,"27":28,"29":30,"31":32,"33":20,"34":35,"36":37,"38":39,"4":5,"41":42,"43":44,"45":46,"47":[48,49,50,51,52,53,54],"6":7,"8":9}],["_id","61a6ac85feb4f0140953d432","about","Cupidatat quis incididunt est laborum dolor deserunt enim eu cupidatat dolor ex sint voluptate tempor. Aliqua commodo culpa dolor occaecat irure commodo est consequat. Lorem aliqua labore aute aute tempor magna consequat sunt proident aliqua. Ullamco fugiat esse elit velit ad consequat aute. Ipsum commodo minim ut consequat culpa labore non sint aute sunt proident veniam esse.\r\n","address","379 Decatur Street, Chalfant, New Hampshire, 8024","age",21,"balance","$3,292.06","company","ZBOO","email","hutchinsonochoa@zboo.com","eyeColor","green","favoriteFruit","apple","friends","id",0,"name","Lynch Conley",1,"Cantu Alford",2,"Hopkins Hill","gender","male","greeting","Hello, Hutchinson Ochoa! You have 2 unread messages.","guid","9ad2b6b6-cce1-442e-80be-c795806625f0","index","isActive",true,"latitude",-72.17581,"longitude",-46.671573,"Hutchinson Ochoa","phone","+1 (828) 470-2742","picture","http://placehold.it/32x32","registered","2020-05-04T02:25:38 -02:00","tags","anim","cillum","elit","aliquip","consectetur","laborum","occaecat"]]"#;

    let result = compacto::decompress_json(input).unwrap();

    let mut output = File::create("compacto/examples/decompress-output.json").unwrap();
    output.write_all(result.to_string().as_bytes()).unwrap();

    println!("file generated: compacto/examples/decompress-output.json");
}
