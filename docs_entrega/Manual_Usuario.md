# Manual de Usuario - Simulador de Gestor de Memoria

---

## 📘 Introducción

### ¿Qué es el Simulador de Gestor de Memoria?

Este simulador es una herramienta que permite visualizar y comprender cómo un Sistema Operativo gestiona la memoria RAM y el área de intercambio (Swap). 

**Utiliza el esquema de Paginación** para asignar memoria a procesos de forma dinámica, mostrando en tiempo real:
- Asignación de marcos de memoria
- Tablas de páginas por proceso
- Algoritmos de reemplazo de páginas (FIFO, LRU, Reloj)
- Métricas de rendimiento

---

## 🚀 Instalación Paso a Paso

### Paso 1: Instalar Rust

El simulador está desarrollado en Rust. Primero necesitas instalar el lenguaje:

#### Windows:

1. Descarga el instalador desde: **https://rustup.rs/**
2. Ejecuta el archivo descargado
3. Sigue las instrucciones en pantalla (opción por defecto está bien)
4. Reinicia la terminal/PowerShell

#### Verificar instalación:

```powershell
rust --version
cargo --version
```

Deberías ver algo como:
```
rustc 1.75.0
cargo 1.75.0
```

### Paso 2: Descargar el Proyecto

Si tienes Git instalado:
```bash
git clone [URL-DEL-REPOSITORIO]
cd memory_sim
```

Si no tienes Git:
1. Descarga el ZIP del repositorio
2. Extrae en una carpeta
3. Abre PowerShell/CMD en esa carpeta

### Paso 3: Compilar

```bash
cargo build --release
```

Esto tardará 1-2 minutos la primera vez. ¡Ten paciencia!

---

## ⚙️ Configuración

Antes de ejecutar, puedes personalizar la configuración editando `config.ini`:

```ini
RAM_SIZE=4096      # Tamaño de RAM en KB
SWAP_SIZE=8192     # Tamaño de Swap en KB
PAGE_SIZE=256      # Tamaño de página en KB
```

### Ejemplos de Configuración

**Configuración Pequeña** (para ver Swap más rápido):
```ini
RAM_SIZE=2048
SWAP_SIZE=4096
PAGE_SIZE=256
```

**Configuración Grande** (más procesos antes de Swap):
```ini
RAM_SIZE=8192
SWAP_SIZE=16384
PAGE_SIZE=512
```

### ⚠️ Notas Importantes

- La división `RAM_SIZE / PAGE_SIZE` debe ser un número entero
- No uses valores menores a 1024 KB para RAM
- `PAGE_SIZE` típicamente es 256, 512, o 1024 KB

---

## 🎮 Ejecutar el Simulador

### Iniciar

```bash
cargo run --release
```

Verás una interfaz como esta:

```
┌─ SIMULADOR DE MEMORIA | ▶️ EJECUTANDO | Algoritmo: FIFO | Página: 256KB ─┐
│                                                                           │
│  [Panel de métricas]              [Mapa de RAM]                           │
│                                                                           │
├─ Logs ────────────────────────────────┬── Swap ─────────────────────────  ┤
│  🟢 NUEVO: P1 'P_1' (512KB, 2 págs)  │  💿 P3 Pág0                       │
│  🔄 SWAP: P2-Pág1 → Swap (Marco 5)   │  💿 P3 Pág1                       │
└────────────────────────────────────────┴──────────────────────────────────┘
```

---

## 🕹️ Guía de Controles

### Controles Básicos

| Tecla | Acción | Descripción |
|-------|--------|-------------|
| **Q** | Salir | Cierra el simulador |
| **P** | Pausar/Reanudar | Pausa la simulación automática |

### Controles Avanzados

| Tecla | Acción | Cuándo usar |
|-------|--------|-------------|
| **N** | Nuevo Proceso | Crear un proceso manualmente para probar |
| **K** | Matar Proceso | Liberar espacio de RAM matando un proceso |
| **1** | Cambiar a FIFO | Probar algoritmo First-In, First-Out |
| **2** | Cambiar a Reloj | Probar algoritmo del Reloj |
| **3** | Cambiar a LRU | Probar Least Recently Used |

### Flujo de Trabajo Recomendado

1. **Iniciar** el simulador (se ejecuta automáticamente)
2. **Observar** durante 30-60 segundos
3. **Pausar** (tecla P) para analizar el estado
4. **Cambiar algoritmo** (teclas 1/2/3) para comparar
5. **Reanudar** (tecla P) y observar diferencias
6. **Salir** (tecla Q) cuando termines

---

## 📊 Interpretar la Interfaz

### 1. Header (Parte Superior)

```
SIMULADOR DE MEMORIA | ▶️ EJECUTANDO | Algoritmo: FIFO | Página: 256KB
```

- **Estado**: EJECUTANDO o PAUSADO
- **Algoritmo activo**: FIFO, LRU, o Reloj
- **Tamaño de página**: Del config.ini

### 2. Panel de Métricas (Izquierda)

#### Gauge de RAM
```
🎯 Uso de RAM
[████████░░] 12/16 marcos (75.0%)
```
- **Verde**: <50% usado
- **Amarillo**: 50-80% usado
- **Rojo**: >80% usado

#### Métricas de Rendimiento
```
📊 MÉTRICAS DE RENDIMIENTO

Fallos de Página: 24
Accesos Totales: 96
Tasa de Fallos: 25.00%

Swaps Realizados: 8
Procesos Creados: 15
Procesos Activos: 6
Procesos Finalizados: 9
```

**¿Qué significan?**

- **Fallos de Página**: Cuántas veces se necesitó hacer Swap
- **Accesos Totales**: Total de operaciones de memoria
- **Tasa de Fallos**: % de accesos que resultaron en fallo
- **Swaps Realizados**: Páginas enviadas al disco
- **Procesos Creados/Activos/Finalizados**: Estadísticas de procesos

#### Panel de Controles
```
🎮 CONTROLES:
Q: Salir
P: Pausar/Reanudar
N: Nuevo proceso
K: Matar proceso
1/2/3: Cambiar algoritmo
```

### 3. Mapa de Memoria RAM (Derecha)

```
🗂️ Mapa de Memoria Física (RAM)
┌────┬────┬────┬─────┐
│P1  │P2  │P3  │LIBRE│
│Pág0│Pág1│Pág0│     │
├────┼────┼────┼─────┤
│P1  │P4  │P4  │LIBRE│
│Pág1│Pág0│Pág1│     │
└────┴────┴────┴─────┘
```

**Cómo leer**:
- Cada cuadro es un **marco físico** de RAM
- **P1, P2, P3**: Proceso que ocupa el marco
- **Pág0, Pág1**: Número de página lógica
- **LIBRE**: Marco disponible
- **Colores**: Cada proceso tiene color único

### 4. Registro de Eventos (Abajo Izquierda)

```
Registro de Eventos
TERMINADO: P5 finalizado y liberado
SWAP: P3-Pág1 → Swap (Marco 8 liberado)
NUEVO: P6 'P_6' (768KB, 3 págs)
```

**Símbolos**:
- **NUEVO**: Proceso creado
- **TERMINADO**: Proceso finalizado
- **SWAP**: Página enviada a Swap
- **Error**: Algo salió mal
- **Sistema PAUSADO/EJECUTANDO**: Cambio de estado

### 5. Cola de Swap (Abajo Derecha)

```
Swap (5/50)
P3 Pág0
P3 Pág1
P5 Pág2
P7 Pág0
P8 Pág1
```

- Muestra páginas actualmente en el área de intercambio
- **(5/50)**: 5 páginas usadas de 50 máximas
- ⚠️ Si llega a 50/50, no se pueden crear más procesos

---

## Preguntas / Dudas

### ¿Por qué aparece "Swap lleno"?

**Respuesta**: El Swap tiene capacidad limitada (50 páginas). Si se llena, nuevos procesos no pueden ser asignados. Solución: Mata procesos (tecla K) o aumenta `SWAP_SIZE` en config.ini.

### ¿Qué algoritmo es mejor?

**Respuesta**: Depende del caso de uso:
- **LRU**: Mejor rendimiento general (menos page faults)
- **Reloj**: Buen balance rendimiento/complejidad
- **FIFO**: Más simple, puede sufrir anomalía de Belady

### ¿Puedo pausar en cualquier momento?

**Respuesta**: Sí, presiona P en cualquier momento para congelar la simulación y analizar el estado.

### ¿Cómo reinicio el simulador?

**Respuesta**: Sal (Q) y ejecuta `cargo run --release` nuevamente.

### ¿Por qué algunos marcos tienen colores?

**Respuesta**: Cada proceso tiene un color aleatorio único para facilitar identificación visual.

### ¿Qué significa "Pág0", "Pág1"?

**Respuesta**: Son los números de página lógica del proceso. Un proceso de 512KB con páginas de 256KB tendrá Pág0 y Pág1.

---

## Solución de Problemas

### El simulador no compila

**Problema**: `cargo build` falla

**Soluciones**:
1. Verificar que Rust esté instalado: `rustc --version`
2. Actualizar Rust: `rustup update`
3. Eliminar carpeta `target/` y volver a compilar

### No se ve la interfaz correctamente

**Problema**: Caracteres raros o layout roto

**Soluciones**:
1. Usar terminal moderno (Windows Terminal recomendado)
2. Maximizar la ventana de terminal
3. Verificar que el terminal soporte UTF-8

### "No se encontró config.ini"

**Problema**: Al iniciar, error de configuración

**Solución**: Asegúrate de estar en la carpeta `memory_sim/` al ejecutar. El archivo `config.ini` debe estar en la misma carpeta.

### El simulador va muy rápido/lento

**Problema**: Difícil seguir eventos

**Soluciones**:
- **Muy rápido**: Pausa frecuentemente (P)
- **Muy lento**: Es normal, los procesos se crean cada ~500ms

---
