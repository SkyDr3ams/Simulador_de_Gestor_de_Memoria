# Estructura del Proyecto - Simulador de Gestor de Memoria

## 📁 Archivos en el Repositorio

```
memory_sim/
├── .gitignore                      # Configuración Git
├── Cargo.toml                      # Dependencias del proyecto
├── Cargo.lock                      # Versiones exactas (reproducibilidad)
├── README.md                       # Documentación principal
├── config.ini                      # Configuración del simulador
│
├── src/                            # Código fuente
│   ├── main.rs                     # Punto de entrada, menús, bucle principal
│   ├── models.rs                   # Lógica del gestor de memoria
│   └── ui.rs                       # Interfaz TUI profesional
│
└── docs_entrega/                   # Documentación para el profesor
    ├── Manual_Usuario.md           # Guía de uso del simulador
    ├── Manual_Tecnico.md           # Documentación técnica detallada
    └── Reporte_Tecnico_Final.md    # Análisis comparativo de algoritmos
```

## 📝 Documentos Incluidos

### Documentación Principal
- **README.md**: Integrantes, instrucciones de compilación, diseño del sistema

### Documentación para Evaluación (docs_entrega/)
1. **Manual_Usuario.md**: Cómo instalar, configurar y usar el simulador
2. **Manual_Tecnico.md**: Arquitectura, estructuras de datos, algoritmos
3. **Reporte_Tecnico_Final.md**: Análisis comparativo de algoritmos (FIFO/Reloj/LRU)

## 🚫 Archivos Excluidos (.gitignore)

- Binarios compilados (`target/`)
- Carpeta de tests (`tests/`)
- Archivos temporales (`*.tmp`, `*.log`)
- Archivos del IDE/editor (`.vscode/`, `.idea/`)
- Archivos del sistema (`.DS_Store`, `Thumbs.db`)

## 📊 Métricas del Proyecto

- **Líneas de código**: ~900 (sin documentación)
- **Algoritmos implementados**: 3 (FIFO, Reloj, LRU)
- **Documentación**: 3 manuales + README
- **Archivos de código**: 3 (main.rs, models.rs, ui.rs)
- **Lenguaje**: Rust 1.70+

## 🎯 Componentes Principales

### Código Fuente (src/)
```
✅ main.rs    (263 líneas) - Configuración, menús, simulación
✅ models.rs  (342 líneas) - Gestor, algoritmos, métricas
✅ ui.rs      (310 líneas) - Interfaz TUI estilo htop
```

### Configuración
```
✅ config.ini  - RAM_SIZE, SWAP_SIZE, PAGE_SIZE
✅ Cargo.toml  - Dependencias (ratatui, crossterm, rand, anyhow)
```

### Documentación
```
✅ README.md                     - Principal
✅ Manual_Usuario.md             - Para usuarios
✅ Manual_Tecnico.md             - Para desarrolladores
✅ Reporte_Tecnico_Final.md      - Para evaluación
```

## 🔧 Tecnologías

- **Lenguaje**: Rust
- **TUI Framework**: Ratatui 0.26
- **Terminal**: Crossterm 0.27
- **Aleatorios**: Rand 0.8
- **Errores**: Anyhow 1.0

## 📦 Comandos Útiles

```bash
# Compilar
cargo build --release

# Ejecutar
cargo run --release

# Limpiar
cargo clean

# Verificar
cargo check
```

---

**Versión**: 1.0  
**Fecha**: Diciembre 2024  
**Proyecto**: Simulador de Gestor de Memoria - Sistemas Operativos
