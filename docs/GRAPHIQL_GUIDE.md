# 📚 GraphiQL Documentation Guide

## 🚀 Accessing GraphiQL

1. **Start the Familiar app**: `./test_visualization.sh` or `cd hot_path && cargo run --release`
2. **Open GraphiQL**: Visit **http://127.0.0.1:8000** in your browser
3. **Wait for loading**: The interface will show "Loading GraphiQL..." briefly, then load the full IDE

## 📖 Finding the Schema Documentation

### **Where is the "Docs" Panel?**

In the **new GraphiQL interface**, schema documentation is accessed differently:

1. **📂 Schema Tab**: Look for a **"Schema"** or **"Docs"** button in the top toolbar
2. **🔍 Explorer Panel**: There should be a sidebar panel that shows schema exploration
3. **⌨️ Auto-completion**: Start typing in the query editor - it will show available fields
4. **🔗 Introspection Query**: You can manually query the schema:

```graphql
{
  __schema {
    types {
      name
      description
      fields {
        name
        description
        type {
          name
        }
      }
    }
  }
}
```

## 🎯 Key Features Now Available

### **1. Auto-completion & IntelliSense**
- Type `{` and see available queries
- Type `mutation {` and see available mutations  
- Ctrl+Space for manual auto-completion

### **2. Schema Explorer**
- Browse all available types and fields
- See field descriptions and examples
- Understand argument requirements

### **3. Query Validation**
- Real-time syntax checking
- Error highlighting
- Type validation

### **4. Documentation in Tooltips**
- Hover over field names to see descriptions
- View argument types and requirements
- See example usage

## 📝 Sample Queries to Try

### **Health Check**
```graphql
{ health }
```

### **Schema Introspection**  
```graphql
{
  __schema {
    queryType { name }
    mutationType { name }
    types { name }
  }
}
```

### **Create Sample Data**
```graphql
mutation {
  createThread(name: "TestPerson", threadType: "person")
  createMoment(text: "A test memory", threadId: "TestPerson")
}
```

## 🛠️ Troubleshooting

### **GraphiQL Not Loading?**
- Check browser console for errors
- Ensure port 8000 is accessible
- Try refreshing the page
- Check if app is running: look for "GraphiQL IDE listening" message

### **No Auto-completion?**
- Make sure you're typing inside the query editor (left panel)
- Try pressing Ctrl+Space manually
- The schema might still be loading - wait a moment

### **Still No Docs Panel?**
- The interface may use different UI than traditional GraphiQL
- Use the introspection query above to explore the schema
- Check for a "Schema" or "Explorer" button in the interface

## 🎨 Enhanced Documentation

The schema now includes:
- **Detailed descriptions** for all mutations
- **Usage examples** in the documentation
- **Argument explanations** for each field
- **Context about the binding-centric model**

## ⚡ Quick Reference

| Action | Shortcut |
|--------|----------|
| Execute Query | Ctrl+Enter |
| Auto-complete | Ctrl+Space |
| Prettify Query | Shift+Ctrl+P |
| Search Schema | Look for search box in docs panel |

The new GraphiQL interface should provide a much better experience for exploring your memory simulation schema! 