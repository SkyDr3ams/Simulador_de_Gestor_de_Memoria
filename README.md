# Simulador de Gestor de Memoria RAM y Swap

## 👥 Integrantes del Equipo
- **[Tu Nombre]** - Matrícula: [Tu Matrícula]
- **[Nombre Compañero 2]** - Matrícula: [Matrícula]
- **[Agregar más según corresponda]**

## 📖 Descripción

Simulador de gestor de memoria RAM y área de intercambio (Swap) de un sistema operativo. El proyecto permite visualizar cómo un SO asigna recursos, traduce direcciones y maneja situaciones de escasez de memoria en un entorno multiprogramado utilizando el esquema de **Paginación**.

### Características Principales

- ✅ **Interfaz profesional** estilo htop sin emojis
- ✅ **Menú interactivo** para seleccionar algoritmo y modo al inicio
- ✅ **Configuración dinámica** desde archivo `config.ini`
- ✅ **Modo automático/manual** - Control total sobre creación de procesos
- ✅ **Límite de 30 procesos** máximo
- ✅ **3 Algoritmos de reemplazo**: FIFO, Reloj, LRU
- ✅ **Paginación completa** con tablas de páginas por proceso
- ✅ **Swapping automático** cuando la RAM se llena
- ✅ **Métricas de rendimiento** en tiempo real
- ✅ **Interfaz TUI** optimizada y compacta

---

## 🚀 Instalación y Ejecución

### Requisitos Previos

- **Rust** 1.70 o superior
- **Cargo** (incluido con Rust)

#### Instalar Rust en Windows

```powershell
# Descargar e instalar desde:
https://rustup.rs/

# O usando winget:
winget install Rustlang.Rust.MSVC
```

### Clonar el Repositorio

```bash
git clone https://github.com/[tu-usuario]/memory_sim.git
cd memory_sim
```

### Configuración

Edita el archivo `config.ini` para ajustar los parámetros del sistema:

```ini
RAM_SIZE=4096      # Tamaño de RAM en KB (4 MB por defecto)
SWAP_SIZE=8192     # Tamaño de Swap en KB (8 MB por defecto)
PAGE_SIZE=256      # Tamaño de página/marco en KB
```

### Compilar y Ejecutar

```bash
cargo run --release
```

Al iniciar, verás un menú interactivo:

```
====================================
  SIMULADOR GESTOR DE MEMORIA v1.0
====================================

Seleccione Algoritmo de Reemplazo:
  1) FIFO (Primero en Entrar, Primero en Salir)
  2) Reloj (Segunda Oportunidad)
  3) LRU (Menos Recientemente Usado)

Ingrese su elección (1-3): _
```

Luego seleccionas el modo de simulación:

```
Seleccione Modo de Simulación:
  1) Automático (procesos creados/terminados automáticamente)
  2) Manual (usted controla creación con tecla 'N')

Ingrese su elección (1-2): _
```

---

## 🖥️ Interfaz del Simulador

### Vista Principal

```
┌─ Gestor de Memoria v1.0 | Algoritmo: FIFO | Modo: AUTOMATICO | Estado: EJECUTANDO ─┐
├──────────────────────── Estadísticas del Sistema ─────────────────────────────────────┤
│  Procesos: 12/30 | Uso RAM: 14/16 marcos (87.5%) | Fallos Página: 42 (26.92%) | ... │
╞════════════════════════════════════════════════════════════════════════════════════════╡
│╔══════ Mapa de Memoria Física (RAM) ═══╗  ┌─ Métricas de Rendimiento ─┐            │
│║ P01 P02 P03 P04 P05 P06 P07 P08       ║  │ Accesos Totales: 156      │            │
│║ P09 P10 P11 P12 -- -- -- --           ║  │ Fallos de Página: 42      │            │
│╚════════════════════════════════════════╝  │ Tasa de Fallos: 26.92%    │            │
│                                            │ Swaps Realizados: 18      │            │
│                                            └───────────────────────────┘            │
│                                            ┌─ Procesos Activos (12) ─┐              │
│                                            │ PID 01 | 512KB | 2/2 pgs│              │
│                                            │ PID 02 | 768KB | 3/3 pgs│              │
├─────── Registro del Sistema ──────┬────────── Área de Swap (8/50) ─────────────────┤
│ [SWAP] Process P3 Page 1 moved    │ PID 03 Página 1                              │
│ [NEW] Process P12 created          │ PID 05 Página 0                              │
│ [TERM] Process P7 terminated       │ PID 07 Página 2                              │
└────────────────────────────────────┴──────────────────────────────────────────────────┘
│ Q:Salir | P:Pausar | N:Nuevo Proceso | K:Matar Proceso | 1/2/3:Algoritmo | A:Cambiar Modo │
└────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Características de la interfaz:**
- Barra superior con estado en tiempo real
- Estadísticas centralizadas en una línea
- Mapa de RAM compacto (8 columnas, 1 línea de altura)
- Panel lateral con métricas y lista de procesos
- Logs limpios sin emojis con etiquetas [CATEGORY]
- Área de Swap visible
- Barra de controles siempre visible

---

## 🎮 Controles Interactivos

| Tecla | Acción | Descripción |
|-------|--------|-------------|
| **Q** | Salir | Cierra el simulador |
| **P** | Pausar | Pausa/reanuda la simulación |
| **A** | Cambiar Modo | Alterna entre automático y manual |
| **N** | Nuevo Proceso | Crea un proceso manualmente (máx. 30) |
| **K** | Matar Proceso | Termina un proceso aleatorio |
| **1** | Algoritmo FIFO | Cambia al algoritmo FIFO |
| **2** | Algoritmo Reloj | Cambia al algoritmo Reloj |
| **3** | Algoritmo LRU | Cambia al algoritmo LRU |

### Modos de Operación

#### Modo Automático
- Procesos se crean y terminan automáticamente cada 500ms
- 60% probabilidad de crear proceso
- 30% probabilidad de terminar proceso
- 10% idle
- **P** pausa la simulación

#### Modo Manual
- Debes presionar **N** para crear cada proceso
- Debes presionar **K** para terminar procesos
- Control total sobre el sistema
- **A** cambia a modo automático en cualquier momento

---

## 📊 Métricas de Rendimiento

El simulador muestra en tiempo real:

| Métrica | Descripción |
|---------|-------------|
| **Procesos** | Activos / Máximo (X/30) |
| **Uso RAM** | Marcos usados / Total (% de utilización) |
| **Fallos de Página** | Total de page faults y porcentaje |
| **Swaps Realizados** | Páginas enviadas al área de intercambio |
| **Accesos Totales** | Total de operaciones de memoria |
| **Tasa de Fallos** | Porcentaje (Fallos / Accesos × 100) |
| **Procesos Creados** | Contador total desde inicio |
| **Procesos Finalizados** | Procesos terminados |

---

## 🏗️ Diseño del Sistema

### Arquitectura de Módulos

```
memory_sim/
├── src/
│   ├── main.rs      → Menú, configuración y bucle principal
│   ├── models.rs    → Lógica del gestor de memoria (core)
│   └── ui.rs        → Interfaz profesional (TUI con Ratatui)
├── config.ini       → Configuración del sistema
├── tests/           → Evidencias de pruebas
- Mapa de RAM con múltiples procesos
- Área de Swap activa
- Comparación de algoritmos (FIFO/Reloj/LRU)
- Panel de métricas con estadísticas

---

## 📚 Documentación Adicional

**Nota**: La carpeta `docs_entrega/` contiene la documentación para entregar al profesor (no se sube a GitHub).

- **Manual de Usuario**: Guía paso a paso para usar el simulador
- **Manual Técnico**: Arquitectura, algoritmos y estructuras de datos
- **Reporte Técnico**: Análisis comparativo y resultados

---

## 🛠️ Dependencias

```toml
[dependencies]
ratatui = "0.26"    # Framework TUI profesional
crossterm = "0.27"  # Control de terminal multiplataforma
rand = "0.8"        # Generación de números aleatorios
anyhow = "1.0"      # Manejo de errores
```

---

## ⚙️ Características Técnicas

- **Lenguaje**: Rust (seguridad de memoria garantizada)
- **Interfaz**: TUI (Text User Interface) con Ratatui
- **Límites del sistema**:
  - Procesos máximos: **30**
  - Swap máximo: **50 páginas**
  - Logs: Últimos **20 eventos**

---

## 📝 Licencia

Proyecto académico desarrollado para el curso de Sistemas Operativos.

---
