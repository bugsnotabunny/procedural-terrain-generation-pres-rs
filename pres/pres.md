Procedural Landscape Generation is a computer graphics technique that involves the automatic and algorithmic creation of virtual landscapes or terrain. Instead of manually designing every aspect of a landscape, developers and designers use mathematical functions and algorithms to generate terrain features, such as mountains, rivers, forests, and valleys, in a highly realistic and efficient manner. Here are some key elements to consider:

Procedural landscape generation relies on a wide range of algorithms and mathematical functions to create terrain. These algorithms are designed to mimic natural processes, such as erosion, weathering, and geological forces. They can produce complex and believable landscapes that exhibit a high degree of realism.

Today we are going to give a talk about some basics of terrain generation algorithms.

To get ourself a visualization of different aproaches we have used Rust programming language, because we are somewhat familiar with it and it provides a good set of libraries for math plotting.

# slide: Flat algorithm

This function does not perform any complex calculations or algorithms. Instead, it simply returns the surface_level that you provide as its third parameter.

# slide: Flat algorithm gif

In the context of landscape generation, this function essentially generates a completely flat landscape at the specified surface_level. It doesn't take into account any variation, terrain features, or procedural generation techniques. It's as if the entire landscape is at a uniform height, as specified by surface_level.

In the natural world, landscapes are rarely perfectly flat. By introducing randomness, you can simulate natural variations in terrain, such as small hills, bumps, and irregularities. This makes the generated landscape look more natural and believable.

Randomness allows for the creation of diverse landscapes. Instead of having a uniform and predictable terrain, each generated landscape becomes unique. This variety can make your virtual worlds more interesting and engaging.

# slide: Absolute random algorithm

The randomness introduced by this function lacks spatial coherence, meaning it doesn't consider the relationship between nearby points. Realistic landscapes exhibit patterns and continuity, which is often absent when randomness is applied without spatial awareness.

# slide: Absolute random algorithm gif

As you can see, this is just a total mess.

To improve this function for landscape generation, we will be using noise functions with controllable randomness later.

As we already seen, a landscape surface level is a kind of a function. Maybe, there is a well-known mathematical function which could provide us some reasonable rerrain?

# slide: Sine curve algorithm

There is. Kind of.

The curve function generates terrain that resembles waves. The undulating pattern can be useful for creating landscapes like rolling hills or ocean surfaces.

# slide: Sine curve gifs

The great adwantage of this solution is customization. The parameters provide customization options for controlling the frequency and amplitude of the waves. This allows you to fine-tune the appearance of the landscape to suit your specific needs.

However, this is not random at all. World like this will bore you by repeating a very same pattern for eternity. This is why more smart algorithms like Perlin noise were invented.

# slide: Perlin noise algorithm

This algorithm uses a Perlin noise function to get some controlled randomness. Perlin noise, unlike std::random function, is continuous, so we won`t have unreasonably steep terrain and spikes all around.

# slide: Perlin noise algorithm gifs

Just like the sine curve function this solution is greatly adjustable due to parameters which allow us to get waves of different length and amplitude.

# slide: How perlin noise works

We are going to talk a bit about how Perlin noise works.

# slide

The idea behind Perlin noise is quite simple. To get ourselves a controllable randomness with some descent graduation we would assing some random values in some reasonable range to our function, using some reasonable step.

# slide

After creating enough points we will interpolate between them, using any method we want for this. This is how we get this random wave-like curve. After all we are able to customize it even more.

# slide

Of course, there are more things to tell about Perlin noise and it`s implementations, but for today we are done. Thank you for your attention.