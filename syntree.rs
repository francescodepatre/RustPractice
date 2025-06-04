use std::collections::HashMap;

trait Expression {
    fn prefix(&self) -> String;

    fn infix(&self) -> String;

    fn eval(&self, context: &HashMap<String, f64>) -> f64;
}

struct BinaryOperation {
    operation: String,
    xValue: Box<dyn Expression>,
    yValue: Box<dyn Expression>,
}

impl Expression for BinaryOperation {
    fn prefix(&self) -> String {
        format!(
            "{} {} {}",
            self.operation,
            self.xValue.prefix(),
            self.yValue.prefix()
        )
    }

    fn infix(&self) -> String {
        format!(
            "{} {} {}",
            self.xValue.infix(),
            self.operation,
            self.yValue.infix()
        )
    }

    fn eval(&self, context: &HashMap<String, f64>) -> f64 {
        let x = self.xValue.eval(context);
        let y = self.yValue.eval(context);

        match self.operation.as_str() {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => x / y,
            _ => panic!("Unknown operation: {}", self.operation),
        }
    }
}

struct UnaryOperation {
    operation: String,
    value: Box<dyn Expression>,
}

impl Expression for UnaryOperation {
    fn prefix(&self) -> String {
        format!("{} {}", self.operation, self.value.prefix())
    }

    fn infix(&self) -> String {
        format!("{} {}", self.value.infix(), self.operation)
    }

    fn eval(&self, context: &HashMap<String, f64>) -> f64 {
        let value = self.eval(context);
        match self.operation.as_str() {
            "~" => -value,
            _ => panic!("Unknown operation: {}", self.operation),
        }
    }
}

struct Variable {
    name: String,
}

impl Expression for Variable {
    fn prefix(&self) -> String {
        format!("{}", self.name)
    }

    fn infix(&self) -> String {
        format!("{}", self.name)
    }

    fn eval(&self, context: &HashMap<String, f64>) -> f64 {
        match context.get(self.name.as_str()) {
            Some(value) => *value,
            None => panic!("Variable {} not found in context", self.name),
        }
    }
}

struct IfExpression {
    condition: Box<dyn Expression>,
    then_branch: Box<dyn Expression>,
    else_branch: Box<dyn Expression>,
}

impl Expression for IfExpression {
    fn prefix(&self) -> String {
        format!(
            "if {} then {} else {}",
            self.condition.prefix(),
            self.then_branch.prefix(),
            self.else_branch.prefix()
        )
    }

    fn infix(&self) -> String {
        format!(
            "if {} then {} else {}",
            self.condition.infix(),
            self.then_branch.infix(),
            self.else_branch.infix()
        )
    }

    fn eval(&self, context: &HashMap<String, f64>) -> f64 {
        if self.condition.eval(context) != 0.0 {
            self.then_branch.eval(context)
        } else {
            self.else_branch.eval(context)
        }
    }
}

struct Number {
    value: f64,
}

impl Expression for Number {
    fn prefix(&self) -> String {
        format!("{}", self.value)
    }

    fn infix(&self) -> String {
        format!("{}", self.value)
    }

    fn eval(&self, context: &HashMap<String, f64>) -> f64 {
        self.value
    }
}

fn main() {
    let context = HashMap::new();

    let expressions = Vec<Box<dyn Expression>> = vec ![
        Box::new(BinaryOperation {
            operation: "+".to_string(),
            xValue: Box::new(Number { value: 2.0 }),
            yValue: Box::new(Number { value: 3.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "*".to_string(),
            xValue: Box::new(Number { value: 4.0 }),
            yValue: Box::new(Number { value: 5.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "-".to_string(),
            xValue: Box::new(Number { value: 10.0 }),
            yValue: Box::new(Number { value: 7.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "/".to_string(),
            xValue: Box::new(Number { value: 20.0 }),
            yValue: Box::new(Number { value: 4.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "*".to_string(),
            xValue: Box::new(Number { value: 3.0 }),
            yValue: Box::new(BinaryOperation {
                operation: "+".to_string(),
                xValue: Box::new(Number { value: 1.0 }),
                yValue: Box::new(Number { value: 2.0 }),
            }),
        }),
        Box::new(BinaryOperation {
            operation: "-".to_string(),
            xValue: Box::new(BinaryOperation {
                operation: "*".to_string(),
                xValue: Box::new(Number { value: 6.0 }),
                yValue: Box::new(Number { value: 6.0 }),
            }),
            yValue: Box::new(Number { value: 10.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "+".to_string(),
            xValue: Box::new(UnaryOperation {
                operation: "~".to_string(),
                value: Box::new(Number { value: 8.0 }),
            }),
            yValue: Box::new(Number { value: 2.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "/".to_string(),
            xValue: Box::new(UnaryOperation {
                operation: "~".to_string(),
                value: Box::new(Number { value: 9.0 }),
            }),
            yValue: Box::new(Number { value: 3.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "*".to_string(),
            xValue: Box::new(IfExpression {
                condition: Box::new(Number { value: 1.0 }),
                then_branch: Box::new(Number { value: 5.0 }),
                else_branch: Box::new(Number { value: 10.0 }),
            }),
            yValue: Box::new(Number { value: 2.0 }),
        }),
        Box::new(BinaryOperation {
            operation: "+".to_string(),
            xValue: Box::new(Variable {
                name: "x".to_string(),
            }),
            yValue: Box::new(Number { value: 3.0 }),
        }),
    ]

    let mut extended_context = context.clone();
    extended_context.insert("x".to_string(), 10.0);

    for (i, expr) in expressions.iter().enumerate() {
        let ctx = if i == 9 { &extended_context } else { &context };
            println!("Espressione {}:", i + 1);
            println!("  Prefisso: {}", expr.prefix());
            println!("  Infisso : {}", expr.infix());
            println!("  Eval    : {}", expr.eval(ctx));
            println!();    }
    
}
