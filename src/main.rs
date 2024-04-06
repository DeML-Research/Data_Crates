
//

use arrow;
use datafusion;
use arrow:array::{BinaryArray, Float64Arrat, UInt16Array, ListArray};
use arrow:datatypes::{DataType, Field, Schema};

use datafusion::execution::context::ExecutionContext;

fn main() {

    // Create a local execution context
    let mut ctx = ExecutionContext::new();

    // Define a schema for the data read from the csv
    let schema = Arc::new(Schema::new(vec![
        Field::new("PassengerID", DataType::Int32, false),
        Field::new("Survived", DataType::Int32, false),
        Field::new("Pclass", DataType::Int32, false),
        Field::new("Name", DataType::Utf8, false),
        Field::new("Sex", DataType::Utf8, false),
        Field::new("Age", DataType::Int32, true),
        Field::new("SibSp", DataType::Int32, false),
        Field::new("Parch", DataType::Int32, false),
        Field::new("Ticket", DataType::Utf8, false),
        Field::new("Fare", DataType::Float64, false), 
        Field::new("Cabin", DataType::Utf8, true),
        Field::new("Embarked", DataType::Utf8, false),
    ]));

}
