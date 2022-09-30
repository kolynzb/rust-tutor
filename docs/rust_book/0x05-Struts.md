# Using Struts To Structure Related Data.

- A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

## Defining and Instantiating Struts.

- Inside curly brackets, we define the names and types of the pieces of data, which we call fields.

`User Strut definition`

```rs
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

`User Strut intance`

```rs
let user1 = User {
email: String::from("someone@example.com"),
username: String::from("someusername123"),
active: true,
sign_in_count: 1,
};
```

- To get a specific value we use dot notation.
  `user.email = String::from("someone@example.com")`
