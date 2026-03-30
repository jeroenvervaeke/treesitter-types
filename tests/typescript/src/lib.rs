include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));

pub fn parse_typescript(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("failed to set TypeScript language");
    parser.parse(src, None).expect("parse failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use treesitter_types::{FromNode, LeafNode, Spanned};

    const COMPLEX_TS: &[u8] = br#"import { readFileSync } from "fs";

interface Identifiable {
    id: string;
}

type Result<T, E = Error> = { ok: true; value: T } | { ok: false; error: E };

interface Repository<T extends Identifiable> {
    findById(id: string): T | undefined;
    save(item: T): void;
}

class InMemoryRepo<T extends Identifiable> implements Repository<T> {
    private items: Map<string, T> = new Map();

    findById(id: string): T | undefined {
        return this.items.get(id);
    }

    save(item: T): void {
        this.items.set(item.id, item);
    }
}

function createRepo<T extends Identifiable>(): Repository<T> {
    return new InMemoryRepo<T>();
}

const defaultId: string = "default";

export { InMemoryRepo, createRepo };
"#;

    #[test]
    fn test_program_children_count() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // import + 2 interfaces + type alias + class + function + const + export = 8
        assert_eq!(program.children.len(), 8);
    }

    #[test]
    fn test_import_statement() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        let ProgramChildren::Statement(stmt) = &program.children[0] else {
            panic!("expected Statement");
        };
        let Statement::ImportStatement(import) = &**stmt else {
            panic!("expected ImportStatement");
        };
        assert!(import.span().start.row == 0);
    }

    #[test]
    fn test_interface_declaration() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Second child is the Identifiable interface
        let ProgramChildren::Statement(stmt) = &program.children[1] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::InterfaceDeclaration(iface) = &**decl else {
            panic!("expected InterfaceDeclaration");
        };
        assert_eq!(iface.name.text(), "Identifiable");
        // No type parameters on this interface
        assert!(iface.type_parameters.is_none());
    }

    #[test]
    fn test_type_alias_with_generics() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Third child is the Result type alias
        let ProgramChildren::Statement(stmt) = &program.children[2] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::TypeAliasDeclaration(type_alias) = &**decl else {
            panic!("expected TypeAliasDeclaration");
        };
        assert_eq!(type_alias.name.text(), "Result");
        // Should have type parameters (T, E = Error)
        let type_params = type_alias
            .type_parameters
            .as_ref()
            .expect("expected type_parameters");
        assert_eq!(type_params.children.len(), 2);
    }

    #[test]
    fn test_generic_interface() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Fourth child: Repository<T extends Identifiable>
        let ProgramChildren::Statement(stmt) = &program.children[3] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::InterfaceDeclaration(iface) = &**decl else {
            panic!("expected InterfaceDeclaration");
        };
        assert_eq!(iface.name.text(), "Repository");
        let type_params = iface
            .type_parameters
            .as_ref()
            .expect("expected type_parameters");
        assert_eq!(type_params.children.len(), 1);
        // No extends clause on the interface itself — `extends` is on the type parameter
        assert!(iface.children.is_none());
    }

    #[test]
    fn test_generic_class() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Fifth child: class InMemoryRepo<T extends Identifiable>
        let ProgramChildren::Statement(stmt) = &program.children[4] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::ClassDeclaration(class) = &**decl else {
            panic!("expected ClassDeclaration");
        };
        assert_eq!(class.name.text(), "InMemoryRepo");
        assert!(class.type_parameters.is_some());
        // implements clause (in class_heritage child)
        assert!(class.children.is_some());
    }

    #[test]
    fn test_generic_function() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Sixth child: function createRepo<T>
        let ProgramChildren::Statement(stmt) = &program.children[5] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::FunctionDeclaration(func) = &**decl else {
            panic!("expected FunctionDeclaration");
        };
        assert_eq!(func.name.text(), "createRepo");
        assert!(func.type_parameters.is_some());
        assert!(func.return_type.is_some());
    }

    #[test]
    fn test_const_declaration() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Seventh child: const defaultId
        let ProgramChildren::Statement(stmt) = &program.children[6] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::LexicalDeclaration(lex_decl) = &**decl else {
            panic!("expected LexicalDeclaration");
        };
        assert_eq!(lex_decl.children.len(), 1);
    }

    #[test]
    fn test_export_statement() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        // Eighth child: export statement
        let ProgramChildren::Statement(stmt) = &program.children[7] else {
            panic!("expected Statement");
        };
        let Statement::ExportStatement(export) = &**stmt else {
            panic!("expected ExportStatement");
        };
        assert!(export.span().start.row > 0);
    }

    #[test]
    fn test_span_covers_source() {
        let tree = parse_typescript(COMPLEX_TS);
        let program = Program::from_node(tree.root_node(), COMPLEX_TS).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte > 0);
        assert!(program.span().end_byte <= COMPLEX_TS.len());
    }
}
