```rs
let (x, y) = (322, 32);
let scale = PxScale { x: 40., y: 40. };
let img_url = "https://cdn.discordapp.com/embed/avatars/3.png";
let file_path = download_avatar(img_url, &tmp_dir).await.unwrap();
let img_builder = get_image_builder(file_path, x, y, "sam", scale);

let img = img_generator.generate(img_builder)?;
img.save("output.png")?;
```