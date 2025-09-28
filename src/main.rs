// example to help get started
fn main(){
	let mut graph:Graph<Layer<NdArray>>=Graph::new();								// create mutable variable storing a new graph of Layer with the NdArray backend. We can use Wgpu later for GPU acceleration, but that's a bit overkill for this example
	let x:Vec<f32>=(0..28*28).map(|n|(n%10) as f32*0.1).collect::<Vec<_>>();			// this is just some vector of 100 floats. It didn't have to be these floats specifically but it is
	let x=Value::from(x);															// create a Value from the vector. Value stores a tensor of either bool, float, int, or multi, or a error, and may have rank from 1-8
				// connect a node labeled "x" to a node labeled "y" using a linear layer. These labels don't have to correspond to variable names, but they are kind of like variable names for the inside of the graph


														// unwrap a rank 1 float tensor from y to print it. (i apparently forgot to implement display for Value, when I do we'll be able to print the value directly)
	graph.connect("x",1_usize).with_clear(true).with(Layer::linear(true,28*28,64,1.0));
	graph.connect(1_usize,2_usize).with_clear(true).with(Layer::relu());
	graph.connect(2_usize,3_usize).with_clear(true).with(Layer::linear(true,64,32,1.0));
	graph.connect(3_usize,4_usize).with_clear(true).with(Layer::relu());
	graph.connect(4_usize,"y").with_clear(true).with(Layer::linear(true,32,10,1.0));

	let y=Unvec(&graph).forward(x);													// apply the graph network by reference to x. AI trait contains the forward method, and is implemented for references too. Since graph can either take a hashmap or vector of inputs, letting it directly have single inputs caused type issues, so Unvec is a convenience wrapper for putting the input in a vec and taking the output out of the vec
	println!("{}",y.unwrap_f1());
}
impl MNIST{
	/// loads the mnist dataset
	pub fn load_training_data()->Self{
		Self{inner:MnistDataset::train()}
	}
	/// loads the mnist dataset
	pub fn load_validation_data()->Self{
		Self{inner:MnistDataset::test()}

	}
}
impl<B:Backend> Dataset < (Value<B>, Value<B>) > for MNIST { // (image, label)
	fn get(&self, index:usize) -> Option <  (Value<B>, Value<B>) > {
		let data = self.inner.get(index)?;
		let image:Vec<f32> = data.image.into_iter().flat_map(|row|row).collect();
		let label = vec![data.label as f32];

		let input = Value::from(image);
		let target = Value::from(label);

		let input = input.reshape([28,28]);

		Some((input, target))
	}

	fn len (&self) -> usize {
		return self.inner.len()
	}
}
struct MNIST{inner:MnistDataset}
use block_graph::{																	// imports from block-graph
	AI,Graph,Unvec,burn::{Layer,Value}
};
use burn::{
	backend::NdArray,data::dataset::{Dataset,vision::MnistDataset},prelude::Backend
};															// imports from burn
