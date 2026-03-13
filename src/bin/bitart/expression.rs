use core::fmt;

use std::rc::Rc;

use rand::Rng;

#[derive(Clone, Debug)]
pub struct Env {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct Literal(i64);
impl Literal {
    fn eval(&self) -> i64 {
        self.0
    }
}
impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Variable {
    X,
    Y,
}
impl Variable {
    const VALUES: &[Variable] = &[Variable::X, Variable::Y];

    fn random<R: Rng>(rng: &mut R) -> Variable {
        let values = Self::VALUES;
        let i = rng.random_range(0..values.len());
        values[i]
    }
    fn eval(&self, env: &Env) -> i64 {
        match self {
            Variable::X => env.x,
            Variable::Y => env.y,
        }
    }
}
impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::X => write!(f, "x"),
            Variable::Y => write!(f, "y"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Negation,
    Complement,
}

impl UnaryOperator {
    const VALUES: &[UnaryOperator] = &[UnaryOperator::Negation, UnaryOperator::Complement];

    fn random<R: Rng>(rng: &mut R) -> UnaryOperator {
        let values = Self::VALUES;
        let i = rng.random_range(0..values.len());
        values[i]
    }
    fn apply(&self, operand: i64) -> i64 {
        match self {
            UnaryOperator::Negation => -operand,
            UnaryOperator::Complement => !operand,
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negation => write!(f, "-"),
            // We use C syntax for `Display`.
            UnaryOperator::Complement => write!(f, "~"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnaryOperation {
    operator: UnaryOperator,
    operand: Expression,
}
impl UnaryOperation {
    fn eval(&self, env: &Env) -> i64 {
        self.operator.apply(self.operand.eval(env))
    }
}
impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.operand)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
}
impl BinaryOperator {
    const VALUES: &[BinaryOperator] = &[
        BinaryOperator::Add,
        BinaryOperator::Sub,
        BinaryOperator::Mul,
        BinaryOperator::Div,
        BinaryOperator::Rem,
        BinaryOperator::And,
        BinaryOperator::Or,
        BinaryOperator::Xor,
    ];

    fn random<R: Rng>(rng: &mut R) -> BinaryOperator {
        let values = Self::VALUES;
        let i = rng.random_range(0..values.len());
        values[i]
    }
    fn apply(&self, operand0: i64, operand1: i64) -> i64 {
        fn safe_div(operand0: i64, operand1: i64) -> i64 {
            if operand1 != 0 {
                operand0 / operand1
            } else {
                0
            }
        }
        fn safe_rem(operand0: i64, operand1: i64) -> i64 {
            if operand1 != 0 {
                operand0 % operand1
            } else {
                0
            }
        }
        match self {
            BinaryOperator::Add => operand0 + operand1,
            BinaryOperator::Sub => operand0 - operand1,
            BinaryOperator::Mul => operand0 * operand1,
            BinaryOperator::Div => safe_div(operand0, operand1),
            BinaryOperator::Rem => safe_rem(operand0, operand1),
            BinaryOperator::And => operand0 & operand1,
            BinaryOperator::Or => operand0 | operand1,
            BinaryOperator::Xor => operand0 ^ operand1,
        }
    }
}
impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Rem => write!(f, "%"),
            BinaryOperator::And => write!(f, "&"),
            BinaryOperator::Or => write!(f, "|"),
            BinaryOperator::Xor => write!(f, "^"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    operator: BinaryOperator,
    operands: [Expression; 2],
}
impl BinaryOperation {
    fn eval(&self, env: &Env) -> i64 {
        self.operator
            .apply(self.operands[0].eval(env), self.operands[1].eval(env))
    }
}
impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.operands[0], self.operator, self.operands[1]
        )
    }
}

#[derive(Clone, Debug)]
pub enum InnerExpression {
    Literal(Literal),
    Variable(Variable),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
}
impl InnerExpression {
    pub fn eval(&self, env: &Env) -> i64 {
        match self {
            InnerExpression::Literal(literal) => literal.eval(),
            InnerExpression::Variable(variable) => variable.eval(env),
            InnerExpression::UnaryOperation(op) => op.eval(env),
            InnerExpression::BinaryOperation(op) => op.eval(env),
        }
    }
}
impl fmt::Display for InnerExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerExpression::Literal(literal) => write!(f, "{}", literal),
            InnerExpression::Variable(variable) => write!(f, "{}", variable),
            InnerExpression::UnaryOperation(op) => write!(f, "{}", op),
            InnerExpression::BinaryOperation(op) => write!(f, "{}", op),
        }
    }
}

pub type Expression = Rc<InnerExpression>;

fn literal(n: i64) -> Expression {
    Rc::new(InnerExpression::Literal(Literal(n)))
}
fn variable(v: Variable) -> Expression {
    Rc::new(InnerExpression::Variable(v))
}
fn unary_operation(operator: UnaryOperator, operand: Expression) -> Expression {
    Rc::new(InnerExpression::UnaryOperation(UnaryOperation {
        operator,
        operand,
    }))
}
fn binary_operation(
    operator: BinaryOperator,
    operand0: Expression,
    operand1: Expression,
) -> Expression {
    let operands = [operand0, operand1];
    Rc::new(InnerExpression::BinaryOperation(BinaryOperation {
        operator,
        operands,
    }))
}

#[derive(Clone, Debug)]
pub struct RandomExpressionBuilder {
    unary_rate: f64,
    variable_rate: f64,
    max_literal: i64,
    depth: usize,
}
impl RandomExpressionBuilder {
    pub fn build<R: Rng>(rng: &mut R) -> Expression {
        let builder = Self::new();
        builder.build_binary(rng, builder.depth)
    }
    fn new() -> RandomExpressionBuilder {
        // Default values
        let unary_rate = 0.3;
        let variable_rate = 0.5;
        let max_literal = 24;
        let depth = 3;
        RandomExpressionBuilder {
            unary_rate,
            variable_rate,
            max_literal,
            depth,
        }
    }
    fn build_recursive<R: Rng>(&self, rng: &mut R, depth: usize, left: bool) -> Expression {
        if depth == 0 {
            self.build_leaf(rng, left)
        } else if rng.random::<f64>() < self.unary_rate {
            self.build_unary(rng, depth)
        } else {
            self.build_binary(rng, depth)
        }
    }
    fn build_leaf<R: Rng>(&self, rng: &mut R, left: bool) -> Expression {
        // Force a variable in a left leaf.
        if left || rng.random::<f64>() < self.variable_rate {
            variable(Variable::random(rng))
        } else {
            literal(rng.random_range(1..=self.max_literal))
        }
    }
    fn build_unary<R: Rng>(&self, rng: &mut R, depth: usize) -> Expression {
        let op = UnaryOperator::random(rng);
        let arg = self.build_recursive(rng, depth - 1, true);
        unary_operation(op, arg)
    }
    fn build_binary<R: Rng>(&self, rng: &mut R, depth: usize) -> Expression {
        let op = BinaryOperator::random(rng);
        let arg0 = self.build_recursive(rng, depth - 1, true);
        let arg1 = self.build_recursive(rng, depth - 1, false);
        binary_operation(op, arg0, arg1)
    }
}
