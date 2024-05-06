```rs
    let (x, y) = (322, 64);
    let big_scale = PxScale { x: 40., y: 40. };
    let small_scale = PxScale { x: 20., y: 20. };
    let img_url = "https://cdn.discordapp.com/avatars/333991818087628810/714e0a6590b4754d4d4fa2b744488bf7.webp?size=1024";
    let file_path = download_avatar(img_url, &tmp_dir).await.unwrap();
    let img_builder = get_image_builder(file_path, x, y, "sam", 20, big_scale, small_scale);

    let img = img_generator.generate(img_builder)?;
    img.save("output.png")?;
```