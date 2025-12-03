# Manual Técnico - Simulador de Gestor de Memoria

---

## 📐 Arquitectura del Sistema

### Visión General

El simulador implementa un modelo simplificado pero funcional de un gestor de memoria basado en **paginación**. Está construido en Rust usando una arquitectura modular de 3 capas:

```
┌────────────────────────────────────┐
│      Capa de Presentación          │
│         (ui.rs - TUI)              │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│      Capa de Lógica de Negocio     │
│     (models.rs - GestorMemoria)    │
└────────────────┬───────────────────┘
                 │
┌────────────────▼───────────────────┐
│      Capa de Configuración         │
│       (main.rs - Config)           │
└────────────────────────────────────┘
```

### Diagrama de Clases (UML Simplificado)

```
┌─────────────────────────────────┐
│          GestorMemoria          │
├─────────────────────────────────┤
│ + marcos_ram: Vec<Marco>        │
│ + cola_swap: VecDeque           │
│ + procesos: Vec<Proceso>        │
│ + algoritmo: AlgoritmoReemplazo │
│ + fallos_pagina: usize          │
│ + accesos_totales: usize        │
├─────────────────────────────────┤
│ + new()                         │
│ + asignar_proceso()             │
│ + matar_proceso_aleatorio()     │
│ - cargar_pagina()               │
│ - reemplazar_pagina()           │
└──────────┬──────────────────────┘
           │
           │ contiene
           │
    ┌──────▼──────┐
    │   Proceso   │
    ├─────────────┤
    │ + pid       │
    │ + paginas   │
    │ + color     │
    └──────┬──────┘
           │
           │ tiene
           │
      ┌────▼─────────┐
      │   Pagina     │ 
      ├──────────────┤
      │ + id         │
      │ + marco_id   │
      │ + ultimo_uso │
      └──────────────┘
```

---

## 🗂️ Estructuras de Datos Detalladas

### 1. `Pagina`

```rust
pub struct Pagina {
    pub id: usize,                // ID de página lógica (0, 1, 2...)
    pub marco_id: Option<usize>,  // Marco físico asignado (None = en Swap)
    pub referenciada: bool,       // Bit R para algoritmo Reloj
    pub ultimo_uso: u64,          // Timestamp para LRU
}
```

**Propósito**: Representa una página lógica en el espacio de direcciones de un proceso.

**Campos**:
- `id`: Identificador único dentro del proceso (índice 0-based)
- `marco_id`: Si es `Some(n)`, la página está en el marco `n` de RAM. Si es `None`, está en Swap
- `referenciada`: Bit de referencia usado por el algoritmo del Reloj
- `ultimo_uso`: Timestamp del último acceso, usado por LRU para determinar víctima

**Estados posibles**:
- **En RAM**: `marco_id = Some(X)`, `referenciada = true`
- **En Swap**: `marco_id = None`, `referenciada = false`

---

### 2. `Marco`

```rust
pub struct Marco {
    pub id: usize,                 // ID del marco físico
    pub proceso_id: Option<usize>, // PID del proceso propietario
    pub pagina_id: Option<usize>,  // ID de página lógica
}
```

**Propósito**: Representa un marco de página física en la RAM.

**Campos**:
- `id`: Identificador único del marco en memoria física
- `proceso_id`: Si es `Some(P)`, el marco está ocupado por el proceso P
- `pagina_id`: Qué página lógica del proceso ocupa este marco

**Estados**:
- **Libre**: `proceso_id = None`, `pagina_id = None`
- **Ocupado**: `proceso_id = Some(P)`, `pagina_id = Some(i)`

**Invariante**: Si `proceso_id.is_some()`, entonces `pagina_id.is_some()` (y viceversa)

---

### 3. `Proceso`

```rust
pub struct Proceso {
    pub pid: usize,              // Process ID único
    pub nombre: String,          // Nombre del proceso
    pub tamaño_kb: usize,        // Tamaño total en KB
    pub paginas: Vec<Pagina>,    // Tabla de páginas
    pub color: (u8, u8, u8),     // Color RGB para visualización
}
```

**Propósito**: Representa un proceso en ejecución con su tabla de páginas.

**Campos**:
- `pid`: Identificador único asignado incrementalmente
- `nombre`: Generado como "P_{pid}"
- `tamaño_kb`: Tamaño en memoria solicitado
- `paginas`: Tabla de páginas del proceso (vector de `Pagina`)
- `color`: Color aleatorio para distinguir visualmente en la UI

**Cálculo de páginas**:
```rust
let paginas_necesarias = (tamaño_kb / tamaño_pagina_kb).ceil();
```

---

### 4. `GestorMemoria`

```rust
pub struct GestorMemoria {
    // Memoria física
    pub marcos_ram: Vec<Marco>,
    
    // Swap
    pub cola_swap: VecDeque<(usize, usize)>,  // (PID, página_id)
    
    // Procesos activos
    pub procesos: Vec<Proceso>,
    
    // Configuración
    pub tamaño_pagina_kb: usize,
    pub algoritmo: AlgoritmoReemplazo,
    pub puntero_reloj: usize,
    
    // Métricas
    pub fallos_pagina: usize,
    pub accesos_totales: usize,
    pub swaps_realizados: usize,
    pub procesos_creados: usize,
    pub procesos_finalizados: usize,
    
    // Logs
    pub logs: Vec<String>,
}
```

**Propósito**: Núcleo del simulador, maneja toda la lógica de gestión de memoria.

**Componentes clave**:
- `marcos_ram`: Array de marcos físicos (tamaño fijo)
- `cola_swap`: Cola FIFO de páginas en área de intercambio
- `procesos`: Lista de procesos activos
- Métricas: Contadores para estadísticas de rendimiento

---

## ⚙️ Algoritmos Implementados

### Algoritmo 1: FIFO (First-In, First-Out)

**Principio**: Reemplazar la página residente en RAM durante más tiempo.

**Implementación**:
```rust
AlgoritmoReemplazo::FIFO => {
    let victima = self.puntero_reloj;
    self.avanzar_reloj(); // Circular pointer
    victima
}
```

**Estructura auxiliar**: 
- `puntero_reloj`: Índice circular que apunta al próximo marco a reemplazar

**Flujo**:
1. Usar puntero actual como víctima
2. Avanzar puntero circularmente: `(puntero + 1) % total_marcos`
3. Retornar índice de víctima

**Ventajas**:
- Simplicidad extrema (O(1))
- Bajo overhead de memoria

**Desventajas**:
- Puede reemplazar páginas frecuentemente usadas
- Sufre de anomal\u00eda de Belady

---

### Algoritmo 2: Reloj (Segunda Oportunidad)

**Principio**: Mejora de FIFO. Da una segunda oportunidad a páginas referenciadas recientemente.

**Implementación**:
```rust
AlgoritmoReemplazo::Reloj => {
    loop {
        let idx = self.puntero_reloj;
        let (pid, pg) = obtener_pagina_en_marco(idx);
        
        if página.referenciada {
            página.referenciada = false;  // Segunda oportunidad
            self.avanzar_reloj();
        } else {
            // No referenciada → víctima
            return idx;
        }
    }
}
```

**Bit de referencia**:
- Se activa (`true`) cuando la página es accedida
- El algoritmo lo desactiva al pasar sobre ella
- Solo reemplaza páginas con bit en `false`

**Flujo**:
1. Examinar página en puntero actual
2. Si `referenciada == true`:
   - Dar segunda oportunidad (poner en `false`)
   - Avanzar puntero
   - Repetir desde 1
3. Si `referenciada == false`:
   - Es la víctima
   - Retornar índice

**Complejidad**: O(n) en peor caso (todas las páginas referenciadas)

---

### Algoritmo 3: LRU (Least Recently Used)

**Principio**: Reemplazar la página que no ha sido usada durante más tiempo.

**Implementación**:
```rust
AlgoritmoReemplazo::LRU => {
    let mut minuso = u64::MAX;
    let mut victima = 0;
    
    for (idx, marco) in marcos_ram.iter().enumerate() {
        if let Some(pid, pag) = (marco.proceso_id, marco.pagina_id) {
            let ultima_usa_pagina = obtener_pagina(pid, pag).ultimo_uso;
            if ultima_usa_pagina < min_uso {
                min_uso = ultima_usa_pagina;
                victima = idx;
            }
        }
    }
    victima
}
```

**Tracking de uso**:
```rust
// En cada acceso a página:
pagina.ultimo_uso = self.accesos_totales;
```

**Flujo**:
1. Iterar sobre todos los marcos
2. Para cada marco ocupado, obtener `ultimo_uso` de su página
3. Seleccionar el marco con menor `ultimo_uso` (menos recientemente usado)
4. Retornar índice

**Complejidad**: O(n) donde n = número de marcos

**Ventajas**:
- Aproximación óptima al algoritmo ideal (OPT)
- Buen rendimiento en práctica

**Desventajas**:
- Overhead de mantener timestamps
- Requiere búsqueda lineal

---

## 🔄 Flujos de Procesos

### Flujo 1: Creación de Proceso

```
[Inicio] → Calcular páginas necesarias
           ↓
         Crear tabla de páginas vacía
           ↓
         Para cada página:
           ├─► Buscar marco libre en RAM
           │   ├─► Si hay libre: Asignar directo
           │   └─► Si RAM llena: Llamar reemplazar_pagina()
           │         ↓
           │       Seleccionar víctima según algoritmo
           │         ↓
           │       Mover víctima a Swap
           │         ↓
           │       Liberar marco
           │         ↓
           │       Asignar al nuevo proceso
           ↓
         Actualizar métricas
           ↓
         Agregar proceso a lista
           ↓
         [Fin]
```

**Pseudocódigo**:
```pascal
PROCEDIMIENTO asignar_proceso(proceso):
    paginas_necesarias ← CEIL(proceso.tamaño / tamaño_pagina)
    
    PARA i ← 0 HASTA paginas_necesarias HACER
        resultado ← cargar_pagina(proceso.pid, i)
        SI resultado = ERROR ENTONCES
            registrar_log("Error al asignar")
            RETORNAR
        FIN SI
    FIN PARA
    
    agregar proceso a lista_procesos
FIN PROCEDIMIENTO
```

---

### Flujo 2: Reemplazo de Página (Swapping)

```
[Page Fault] → ¿Hay marcos libres?
                ├─► SÍ: Usar marco libre → [Fin]
                └─► NO: Continuar
                      ↓
                Ejecutar algoritmo de reemplazo
                      ↓
                Obtener índice de víctima
                      ↓
                Extraer (pid_victima, pag_victima)
                      ↓
                Actualizar tabla: 
                  víctima.marco_id = None
                      ↓
                ¿Swap tiene espacio?
                ├─► NO: Error "Swap lleno" → [Fin]
                └─► SÍ: Continuar
                      ↓
                Agregar (pid, pag) a cola_swap
                      ↓
                Incrementar contador swaps_realizados
                      ↓
                Liberar marco (proceso_id = None)
                      ↓
                Retornar índice del marco liberado
                      ↓
                [Fin]
```

**Pseudocódigo**:
```
FUNCIÓN reemplazar_pagina() → índice_marco:
    // Seleccionar víctima según algoritmo
    SEGÚN algoritmo_activo HACER
        CASO FIFO:
            idx_victima ← puntero_reloj
            avanzar_puntero_circular()
        CASO Reloj:
            REPETIR
                SI marcos[idx].página.referenciada ENTONCES
                    marcos[idx].página.referenciada ← FALSO
                    avanzar_puntero()
                SINO
                    SALIR DE BUCLE
                FIN SI
            FIN REPETIR
        CASO LRU:
            idx_victima ← buscar_menor_ultimo_uso()
    FIN SEGÚN
    
    // Realizar swap
    pid, pag ← obtener_datos_marco(idx_victima)
    actualizar_tabla(pid, pag, marco ← NONE)
    agregar_a_swap(pid, pag)
    
    RETORNAR idx_victima
FIN FUNCIÓN
```

---

### Flujo 3: Terminación de Proceso

```
[kill_random_process() llamado]
           ↓
    Seleccionar proceso aleatorio (RNG)
           ↓
    pid_victima ← proceso.pid
           ↓
    Para cada marco en RAM:
      ├─► Si marco.proceso_id == pid_victima
      │     └─► Liberar: marco.proceso_id = None
      │                   marco.pagina_id = None
      └─► Continuar
           ↓
    Para cada (pid, pag) en cola_swap:
      ├─►Si pid == pid_victima
      │     └─► Eliminar de cola
      └─► Continuar
           ↓
    Eliminar proceso de lista_procesos
           ↓
    Incrementar procesos_finalizados
           ↓
    Registrar log: "💀 TERMINADO: P{pid}"
           ↓
    [Fin]
```

---

## 📊 Cálculo de Métricas

### Tasa de Fallos de Página

```rust
pub fn tasa_fallos(&self) -> f64 {
    if self.accesos_totales == 0 {
        return 0.0;
    }
    (self.fallos_pagina as f64 / self.accesos_totales as f64) * 100.0
}
```

**Fórmula**:
```
Tasa = (Fallos de Página / Accesos Totales) × 100
```

**Ejemplo**:
- Accesos: 250
- Fallos: 62
- Tasa = (62/250) × 100 = **24.8%**

### Utilización de RAM

```rust
pub fn utilización_ram(&self) -> f64 {
    let total = self.marcos_ram.len();
    if total == 0 { return 0.0; }
    
    let usados = total - self.contar_marcos_libres();
    (usados as f64 / total as f64) * 100.0
}
```

**Ejemplo**:
- Total marcos: 16
- Marcos libres: 3
- Usados: 16 - 3 = 13
- Utilización = (13/16) × 100 = **81.25%**

---

## 🎨 Capa de Presentación (TUI con Ratatui)

### Arquitectura de UI

La interfaz usa **Ratatui** (Rust TUI framework) con **Crossterm** para control de terminal.

```rust
// Layout jerárquico
Pantalla completa
├─ Header (3 líneas)
├─ Body (expandible)
│  ├─ Panel Métricas (30%)
│  └─ Mapa RAM (70%)
└─ Footer (12 líneas)
   ├─ Logs (60%)
   └─ Swap (40%)
```

### Renderizado

**Ciclo de renderizado**:
```rust
loop {
    terminal.draw(|f| {
        ui::dibujar(f, &gestor, pausado);
    })?;
    
    // Procesar eventos de teclado
    // Actualizar estado
}
```

**Frecuencia**: 100ms por tick (10 FPS)

### Colores Dinámicos

Cada proceso recibe un color RGB aleatorio:
```rust
let (r, g, b) = (
    rng.gen_range(50..255),
    rng.gen_range(50..255),
    rng.gen_range(50..255)
);
```

**Rango 50-255**: Evita colores muy oscuros para legibilidad.

---

## 🧪 Casos de Prueba Técnicos

### Test 1: Verificación de Invariantes

**Invariante**: Si una página tiene `marco_id = Some(X)`, entonces el marco X debe tener `proceso_id = Some(P)` donde P es el proceso dueño de esa página.

**Código de verificación**:
```rust
fn verificar_consistencia(gestor: &GestorMemoria) -> bool {
    for proceso in &gestor.procesos {
        for pagina in &proceso.paginas {
            if let Some(marco_id) = pagina.marco_id {
                let marco = &gestor.marcos_ram[marco_id];
                if marco.proceso_id != Some(proceso.pid) {
                    return false;  // Inconsistencia!
                }
            }
        }
    }
    true
}
```

### Test 2: Límite de Swap

**Escenario**: Llenar Swap hasta 50 páginas y verificar rechazo.

**Pasos**:
1. Configurar RAM muy pequeña (4 marcos)
2. Crear procesos hasta saturar Swap
3. Verificar mensaje: "Swap lleno (50 páginas MAX)"
4. Confirmar que `cola_swap.len() == 50`

---

## 🔧 Optimizaciones Implementadas

### 1. Búsqueda de marcos libres O(n)

```rust
// versión optimizada con iterator
let marco_libre = self.marcos_ram.iter()
    .position(|marco| marco.proceso_id.is_none());
```

### 2. Uso de VecDeque para Swap

`VecDeque` permite push_back() y pop_front() en O(1), ideal para cola FIFO.

### 3. Limitación de Logs

```rust
if self.logs.len() >= 20 {
    self.logs.remove(0);  // FIFO para logs
}
```

Evita crecimiento ilimitado de memoria.

---

## 📚 Dependencias y Justificación

| Crate | Versión | Uso | Justificación |
|-------|---------|-----|---------------|
| `ratatui` | 0.26 | Framework TUI | Mejor framework para Terminal UI en Rust |
| `crossterm` | 0.27 | Control de terminal | Multiplataforma (Windows/Linux/Mac) |
| `rand` | 0.8 | Aleatoriedad | Generar procesos y colores |
| `anyhow` | 1.0 | Manejo de errores | Simplifica propagación de errores |

---

## 🔮 Extensiones Posibles

1. **Persistencia**: Guardar/cargar estado del simulador
2. **TLB**: Agregar Translation Lookaside Buffer
3. **Paginación multinivel**: Implementar tablas de 2 niveles
4. **Segmentación**: Combinar paginación con segmentación
5. **Demand Paging**: Cargar páginas solo cuando se acceden
6. **Working Set**: Implementar modelo de conjunto de trabajo

---
