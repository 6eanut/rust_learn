// option也是枚举
// 有some和none两个结果
// rust里面没有null，因为null会产生许多不安全的操作
// option的出现就是为了表达null
// 就像下面的例子，x和y无法相加
// x一定不为null，但是option的y可能为空，要是想做操作，需要把option进行转化，这就需要判断
fn main() {
    let x=5;
    let y = Some((5));
    let z=x+y;
}
