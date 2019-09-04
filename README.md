# RTC-RayTracer

A simple ray tracer

Gif of images created by the raytracer:

![Planet](./media/Planet.gif)

The shapes could use some refactoring.
I already tried the following things / thought about them:

1. `Shape` struct that holds a pair of functions pointers with those pointers defining the shape through surface-normal and intersection. Failed because of higher-ranked trait bounds I couldn't resolve. Would be easily extendable and also allow all shapes to have the same type. I imagine it to be comparatively slow though because it has to go through the function pointer each time. Also requires all usage sites to explicitly pass the `&Shape` which seems a bit clumsy.
2. `Shape` struct that's generic over empty types like `enum Sphere {}` and then implementing a `Render` trait for `Shape<Sphere>` to add specific functionality. Seems really nice and is easily extendable but results in seperate types for all shapes. `Intersections` already posed a problem in storage. Fine for homogenous storage of shapes - no idea how to do it for heterogenous one though. Maybe implementing a trait for all `Shape<T>` that are `Render` and abstracting fields into the trait interface. Requires trait objects though which I'm not a fan of. In the non trait-object variant probably very performant.
3. `Shape` struct with `ShapeType` enum that determines the kind of shape. Works but isn't externally extendable / requires to touch old code on changes. Also produces rather ugly though simple code. Essentially a handrolled dynamic dispatch.
4. Essentially going with the `Shape<T>`-generic-over-empty-types approach but making seperate storages for all `T` on `World` etc.. Has the benefit of pure static dispatch and is thus probably very fast, but requires changes to all places that require such storage on changes to the shapes and is absolutely not extendable externally.
