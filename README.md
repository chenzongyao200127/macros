# Rust 宏

## 语法
- $x: item - an item, like a function, struct, module, etc.
- $x: block - a block ({...})
- $x: stmt - a statement
- $x: pat - a pattern


## procedure marco 构建过程宏

noromal procedure: Type -> process -> Type
macro: `TokenStream` -> (extract) -> `data` -> `quote!{}` -> `TokenStream`


~~~rust
DeriveInput {
    attrs: [],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Direction",
        span: #0 bytes(55..64),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Data::Enum {
        enum_token: Enum,
        brace_token: Brace,
        variants: [
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Up",
                    span: #0 bytes(71..73),
                },
                fields: Fields::Unnamed {
                    paren_token: Paren,
                    unnamed: [
                        Field {
                            attrs: [],
                            vis: Visibility::Inherited,
                            mutability: FieldMutability::None,
                            ident: None,
                            colon_token: None,
                            ty: Type::Path {
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "DirectionUp",
                                                span: #0 bytes(74..85),
                                            },
                                            arguments: PathArguments::None,
                                        },
                                    ],
                                },
                            },
                        },
                    ],
                },
                discriminant: None,
            },
            Comma,
            Variant {
                attrs: [],
                ident: Ident {
                    ident: "Down",
                    span: #0 bytes(92..96),
                },
                fields: Fields::Unit,
                discriminant: None,
            },
            Comma,
        ],
    },
}

impl From<DirectionUp> for Direction {
    fn from(v: DirectionUp) -> Self {
        Direction::Up(v)
    }
}
impl From<u32> for Direction {
    fn from(v: u32) -> Self {
        Direction::Left(v)
    }
}
~~~
