

use std::collections::{HashMap};

use super::layer::Layer;
use super::activation::Activation;
use super::nodetype::NodeType;


/// Neuron represents a node in a nerual network graph. It holds
/// an innovation number to help edges in the network identify which
/// node it's pointing to, a value which is its activated value 
/// a node type, being either input, hidden, or output, a vec of outgoing 
/// numbers. The output numbers are the innovation nmbers of the edges that
/// connect this node to another node (meaning this node is the egde's src node)
/// this lets us traverse the network quickly and simply while also keeping
/// track of the weights and active flags of the connections. Incoming keeps 
/// track of the nodes this node is expecting inputs from, the key is the innovation
/// number of the node it is expecting input from, and the value is that input
#[derive(Debug)]
pub struct Neuron {
    pub innov: i32,
    pub curr_value: Option<f64>,
    pub prev_value: Option<f64>,
    pub cell_state: Option<f64>,
    pub layer_type: Layer,
    pub node_type: NodeType,
    pub activation: Activation,
    pub outgoing: Vec<i32>,
    pub incoming: HashMap<i32, Option<f64>>
}



/// implement the neuron
impl Neuron {

    /// return a blank neuron with only a innov and node type, everything
    /// else is completely empty
    pub fn new(innov: i32, layer_type: Layer, node_type: NodeType, activation: Activation) -> Self {
        Neuron {
            innov,
            curr_value: None,
            prev_value: None,
            cell_state: None,
            layer_type,
            node_type,
            activation,
            outgoing: Vec::new(),
            incoming: HashMap::new()
        }
    }



    /// Turn the neuron into a raw mutable pointer - this
    /// makes the data structure inherintly unsafe 
    pub fn as_mut_ptr(self) -> *mut Neuron {
        Box::into_raw(Box::new(self))
    }



    /// Activate the neuron by testing to see if first it can be activated,
    /// meaning it has gotten all its expected inputs, and if it does 
    /// activate the sum of those inputs and assign it to the value of the neuron
    /// If the neuron was activated, return true, else false
    #[inline]
    pub fn is_ready(&mut self) -> bool {
        let can_activate = self.incoming.values().all(|x| x.is_some());
        if can_activate {
            let (temp_cell_state, temp_curr_value) = self.node_type.activate(&self.incoming, &self.activation, &self.prev_value, &self.cell_state);
            self.curr_value = temp_curr_value;
            self.cell_state = temp_cell_state;
            return true;
        }
        false
    }



    /// reset the values in the neurons incoming nodes and its value 
    #[inline]
    pub fn reset_node(&mut self) {
        self.prev_value = self.curr_value.clone();
        self.curr_value = None;
        self.cell_state = None;
        for (_, val) in self.incoming.iter_mut() {
            *val = None;
        }
    }


}


/// implement clone for a neuron
impl Clone for Neuron {
    fn clone(&self) -> Self {
        Neuron {
            innov: self.innov,
            curr_value: None,
            prev_value: None,
            cell_state: None,
            layer_type: self.layer_type,
            node_type: self.node_type,
            activation: self.activation,
            outgoing: self.outgoing
                .iter()
                .map(|x| *x)
                .collect::<Vec<_>>(),
            incoming: self.incoming
                .iter()
                .map(|(key, val)| (*key, *val))
                .collect::<HashMap<_, _>>()
        }
    }
}