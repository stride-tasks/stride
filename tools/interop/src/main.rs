#![allow(clippy::unnecessary_wraps)]

use std::{
    any::TypeId,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Write,
};

use anyhow::bail;
use facet::{ConstTypeId, Facet, Shape, StructType, UserType};
use stride_core::backend::BackendRecord;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    // let shape = BackendRecord::SHAPE;

    // println!("{:#?}", shape);

    let mut context = Context::new([BackendRecord::SHAPE]);

    match context.process() {
        Ok(output) => println!("{output}"),
        Err(e) => {
            println!("{}", context.output);
            return Err(e);
        }
    }

    Ok(())
}

struct DartType {
    name: String,
    module: Option<String>,
}

struct Resolved {
    shape: &'static Shape,
    dart: DartType,
}

struct Context {
    queue: VecDeque<&'static Shape>,
    resolved: HashMap<TypeId, Resolved>,

    imports: HashSet<String>,
    output: String,
}

impl Context {
    fn new(shapes: impl IntoIterator<Item = &'static Shape>) -> Self {
        let mut resolved = HashMap::new();

        resolved.insert(
            TypeId::of::<Uuid>(),
            Resolved {
                shape: Uuid::SHAPE,
                dart: DartType {
                    name: "UuidValue".into(),
                    module: Some("package:uuid/uuid.dart".into()),
                },
            },
        );
        resolved.insert(
            TypeId::of::<Box<str>>(),
            Resolved {
                shape: Box::<str>::SHAPE,
                dart: DartType {
                    name: "String".into(),
                    module: None,
                },
            },
        );
        resolved.insert(
            TypeId::of::<bool>(),
            Resolved {
                shape: bool::SHAPE,
                dart: DartType {
                    name: "bool".into(),
                    module: None,
                },
            },
        );

        Self {
            queue: shapes.into_iter().collect(),
            resolved,
            imports: HashSet::new(),
            output: String::new(),
        }
    }

    fn process(&mut self) -> anyhow::Result<String> {
        while let Some(shape) = self.queue.pop_back() {
            self.handle_shape(shape)?;
        }

        let mut imports = self.imports.iter().fold(String::new(), |mut acc, module| {
            acc.push_str("import '");
            acc.push_str(module);
            acc.push_str("';\n");
            acc
        });

        imports.push_str("\n\n");
        imports.push_str(&self.output);
        Ok(imports)
    }

    fn handle_shape(&mut self, shape: &'static Shape) -> anyhow::Result<()> {
        match shape.ty {
            facet::Type::User(user) => self.handle_user_type(shape, &user)?,
            facet::Type::Primitive(primitive_type) => todo!(),
            facet::Type::Sequence(sequence_type) => todo!(),
            facet::Type::Pointer(pointer_type) => todo!(),
        }
        Ok(())
    }

    fn handle_user_type(&mut self, shape: &'static Shape, ty: &UserType) -> anyhow::Result<()> {
        match ty {
            UserType::Struct(ty) => self.handle_struct(shape, ty)?,
            UserType::Enum(ty) => bail!("unions are not supported"),
            UserType::Union(ty) => bail!("unions are not supported"),
            UserType::Opaque => todo!("{}", shape.type_identifier),
        }
        Ok(())
    }

    fn handle_struct(&mut self, shape: &'static Shape, ty: &StructType) -> anyhow::Result<()> {
        let name = shape.type_identifier;

        self.resolved.insert(
            shape.id.get(),
            Resolved {
                shape,
                dart: DartType {
                    name: name.into(),
                    module: None,
                },
            },
        );

        writeln!(&mut self.output, "class {name} {{")?;
        for field in ty.fields {
            let type_name = if let Some(value) = self.resolved.get(&field.shape.id.get()) {
                if let Some(module) = &value.dart.module {
                    if !self.imports.contains(module) {
                        self.imports.insert(module.clone());
                    }
                }

                value.dart.name.clone()
            } else {
                self.queue.push_front(field.shape);

                field.shape.type_identifier.into()
            };

            writeln!(&mut self.output, "    final {} {};", type_name, field.name)?;
        }
        writeln!(&mut self.output, "}}\n")?;
        Ok(())
    }
}
