use rand::seq::SliceRandom;
use std::collections::VecDeque;

// --- ENUMERACIONES Y ESTRUCTURAS DE DATOS ---

#[derive(Clone, Debug, PartialEq)]
pub enum AlgoritmoReemplazo {
    FIFO,  // First In First Out
    LRU,   // Least Recently Used
    Reloj, // Algoritmo del Reloj (segunda oportunidad)
}

/// Representa una página lógica de un proceso
#[derive(Clone, Debug, PartialEq)]
pub struct Pagina {
    pub id: usize,               // ID de la página lógica
    pub marco_id: Option<usize>, // Marco físico asignado (None = en Swap)
    pub referenciada: bool,      // Bit de referencia (para algoritmo Reloj)
    pub ultimo_uso: u64,         // Timestamp del último acceso (para LRU)
}

/// Representa un proceso en el sistema
#[derive(Clone, Debug)]
pub struct Proceso {
    pub pid: usize,           // Process ID único
    pub nombre: String,       // Nombre del proceso
    pub tamaño_kb: usize,     // Tamaño total en KB
    pub paginas: Vec<Pagina>, // Tabla de páginas del proceso
    pub color: (u8, u8, u8),  // Color RGB para visualización
}

/// Representa un marco de página física en RAM
#[derive(Clone, Debug, PartialEq)]
pub struct Marco {
    pub id: usize,                 // ID del marco físico
    pub proceso_id: Option<usize>, // PID del proceso que lo ocupa (None = libre)
    pub pagina_id: Option<usize>,  // ID de página lógica asignada
}

/// Gestor principal de memoria RAM y Swap
pub struct GestorMemoria {
    pub marcos_ram: Vec<Marco>,              // Memoria física (RAM)
    pub cola_swap: VecDeque<(usize, usize)>, // Cola de Swap: (PID, página)
    pub procesos: Vec<Proceso>,              // Procesos activos en el sistema
    pub tamaño_pagina_kb: usize,             // Tamaño de página/marco en KB
    pub logs: Vec<String>,                   // Registro de eventos
    pub algoritmo: AlgoritmoReemplazo,       // Algoritmo de reemplazo activo
    pub puntero_reloj: usize,                // Puntero para algoritmo Reloj y FIFO

    // --- MÉTRICAS DE RENDIMIENTO ---
    pub fallos_pagina: usize,        // Total de page faults
    pub accesos_totales: usize,      // Total de accesos a memoria
    pub swaps_realizados: usize,     // Número de páginas enviadas a Swap
    pub procesos_creados: usize,     // Total de procesos creados
    pub procesos_finalizados: usize, // Total de procesos terminados
}

impl GestorMemoria {
    /// Crea un nuevo gestor de memoria con la configuración especificada
    pub fn new(
        tamaño_ram: usize,
        tamaño_swap: usize,
        tamaño_pagina: usize,
        algoritmo: AlgoritmoReemplazo,
    ) -> Self {
        let total_marcos = tamaño_ram / tamaño_pagina;
        let marcos = (0..total_marcos)
            .map(|i| Marco {
                id: i,
                proceso_id: None,
                pagina_id: None,
            })
            .collect();

        Self {
            marcos_ram: marcos,
            cola_swap: VecDeque::new(),
            procesos: Vec::new(),
            tamaño_pagina_kb: tamaño_pagina,
            logs: vec![format!(
                "[INICIO] Sistema iniciado - Algoritmo: {:?} | RAM: {}KB | Swap: {}KB",
                algoritmo, tamaño_ram, tamaño_swap
            )],
            algoritmo,
            puntero_reloj: 0,
            // Inicializar métricas
            fallos_pagina: 0,
            accesos_totales: 0,
            swaps_realizados: 0,
            procesos_creados: 0,
            procesos_finalizados: 0,
        }
    }

    /// Registra un mensaje en el log del sistema (mantiene últimos 20)
    pub fn registrar_log(&mut self, mensaje: &str) {
        if self.logs.len() >= 20 {
            self.logs.remove(0);
        }
        self.logs.push(mensaje.to_string());
    }

    /// Cuenta cuántos marcos están libres en RAM
    pub fn contar_marcos_libres(&self) -> usize {
        self.marcos_ram
            .iter()
            .filter(|marco| marco.proceso_id.is_none())
            .count()
    }

    /// Calcula el porcentaje de utilización de RAM
    pub fn utilización_ram(&self) -> f64 {
        let total = self.marcos_ram.len();
        if total == 0 {
            return 0.0;
        }
        let usados = total - self.contar_marcos_libres();
        (usados as f64 / total as f64) * 100.0
    }

    /// Calcula la tasa de fallos de página
    pub fn tasa_fallos(&self) -> f64 {
        if self.accesos_totales == 0 {
            return 0.0;
        }
        (self.fallos_pagina as f64 / self.accesos_totales as f64) * 100.0
    }

    /// Avanza el puntero del reloj (para FIFO y algoritmo Reloj)
    fn avanzar_reloj(&mut self) {
        let len = self.marcos_ram.len();
        if len > 0 {
            self.puntero_reloj = (self.puntero_reloj + 1) % len;
        }
    }

    /// Obtiene referencia mutable a una página específica de un proceso
    fn obtener_info_pagina(&mut self, pid: usize, idx_pagina: usize) -> Option<&mut Pagina> {
        if let Some(proceso) = self.procesos.iter_mut().find(|p| p.pid == pid) {
            proceso.paginas.iter_mut().find(|p| p.id == idx_pagina)
        } else {
            None
        }
    }

    /// Asigna un nuevo proceso al sistema (Requisito B y C)
    pub fn asignar_proceso(&mut self, mut proceso: Proceso) {
        let paginas_necesarias =
            (proceso.tamaño_kb as f64 / self.tamaño_pagina_kb as f64).ceil() as usize;

        // Crear tabla de páginas del proceso
        for i in 0..paginas_necesarias {
            proceso.paginas.push(Pagina {
                id: i,
                marco_id: None,
                referenciada: true,
                ultimo_uso: 0,
            });
        }

        self.procesos_creados += 1;
        self.registrar_log(&format!(
            "[NUEVO] Proceso P{} '{}' creado ({}KB, {} páginas)",
            proceso.pid, proceso.nombre, proceso.tamaño_kb, paginas_necesarias
        ));

        // Intentar cargar todas las páginas del proceso
        for idx_pagina in 0..paginas_necesarias {
            if let Err(error) = self.cargar_pagina(proceso.pid, idx_pagina) {
                self.registrar_log(&format!("[ERROR] {}", error));
                return;
            }
        }

        self.procesos.push(proceso);
    }

    /// Mata un proceso aleatorio y libera sus recursos (Requisito B)
    pub fn matar_proceso_aleatorio(&mut self) {
        if self.procesos.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let pid_victima = self.procesos.choose(&mut rng).unwrap().pid;

        // Liberar marcos RAM ocupados por el proceso
        for marco in &mut self.marcos_ram {
            if marco.proceso_id == Some(pid_victima) {
                marco.proceso_id = None;
                marco.pagina_id = None;
            }
        }

        // Limpiar páginas del proceso en Swap
        let mut nueva_cola_swap = VecDeque::new();
        while let Some((pid, pagina)) = self.cola_swap.pop_front() {
            if pid != pid_victima {
                nueva_cola_swap.push_back((pid, pagina));
            }
        }
        self.cola_swap = nueva_cola_swap;

        // Eliminar proceso de la lista
        self.procesos.retain(|p| p.pid != pid_victima);
        self.procesos_finalizados += 1;

        self.registrar_log(&format!(
            "[TERM] Proceso P{} terminado y liberado",
            pid_victima
        ));
    }

    /// Carga una página en memoria RAM (con swapping si es necesario)
    fn cargar_pagina(&mut self, pid: usize, idx_pagina: usize) -> Result<(), String> {
        self.accesos_totales += 1;
        let tiempo_acceso = self.accesos_totales as u64; // Capturar antes del préstamo mutable

        // Buscar marco libre en RAM
        let marco_libre = self
            .marcos_ram
            .iter()
            .position(|marco| marco.proceso_id.is_none());

        let idx_objetivo = match marco_libre {
            Some(i) => {
                // Hay espacio libre
                i
            }
            None => {
                // RAM llena, activar algoritmo de reemplazo (Requisito E)
                self.fallos_pagina += 1;
                self.reemplazar_pagina()?
            }
        };

        // Asignar página al marco
        self.marcos_ram[idx_objetivo].proceso_id = Some(pid);
        self.marcos_ram[idx_objetivo].pagina_id = Some(idx_pagina);

        // Actualizar tabla de páginas del proceso
        if let Some(pagina) = self.obtener_info_pagina(pid, idx_pagina) {
            pagina.marco_id = Some(idx_objetivo);
            pagina.referenciada = true;
            pagina.ultimo_uso = tiempo_acceso; // Usar valor capturado
        }

        // Avanzar puntero si es FIFO
        if self.algoritmo == AlgoritmoReemplazo::FIFO {
            self.avanzar_reloj();
        }

        Ok(())
    }

    /// Selecciona y reemplaza una página víctima usando el algoritmo configurado (Requisito E)
    fn reemplazar_pagina(&mut self) -> Result<usize, String> {
        let idx_victima = match self.algoritmo {
            // FIFO: Reemplaza la página más antigua (puntero circular)
            AlgoritmoReemplazo::FIFO => self.puntero_reloj,

            // Algoritmo del Reloj: Segunda oportunidad
            AlgoritmoReemplazo::Reloj => {
                loop {
                    let idx = self.puntero_reloj;
                    let (pid, pg) = {
                        let marco = &self.marcos_ram[idx];
                        (marco.proceso_id.unwrap(), marco.pagina_id.unwrap())
                    };

                    let mut es_victima = false;
                    if let Some(pagina) = self.obtener_info_pagina(pid, pg) {
                        if pagina.referenciada {
                            // Dar segunda oportunidad
                            pagina.referenciada = false;
                        } else {
                            // No ha sido referenciada, es víctima
                            es_victima = true;
                        }
                    }

                    if es_victima {
                        break idx;
                    } else {
                        self.avanzar_reloj();
                    }
                }
            }

            // LRU: Reemplaza la página menos recientemente usada
            AlgoritmoReemplazo::LRU => {
                let mut min_uso = u64::MAX;
                let mut idx_victima = 0;

                for (idx, marco) in self.marcos_ram.iter().enumerate() {
                    if let (Some(pid), Some(pg)) = (marco.proceso_id, marco.pagina_id) {
                        if let Some(pagina) = self
                            .procesos
                            .iter()
                            .find(|p| p.pid == pid)
                            .and_then(|p| p.paginas.iter().find(|page| page.id == pg))
                        {
                            if pagina.ultimo_uso < min_uso {
                                min_uso = pagina.ultimo_uso;
                                idx_victima = idx;
                            }
                        }
                    }
                }
                idx_victima
            }
        };

        // Obtener información de la página víctima
        let (pid_victima, pag_victima) = {
            let marco = &self.marcos_ram[idx_victima];
            (marco.proceso_id.unwrap(), marco.pagina_id.unwrap())
        };

        // Actualizar tabla de páginas (marcar como no presente en RAM)
        if let Some(pagina) = self.obtener_info_pagina(pid_victima, pag_victima) {
            pagina.marco_id = None;
        }

        // Verificar capacidad del Swap
        if self.cola_swap.len() >= 50 {
            return Err("[ERROR] Swap lleno (50 páginas MAX)".to_string());
        }

        // Mover página a Swap
        self.cola_swap.push_back((pid_victima, pag_victima));
        self.swaps_realizados += 1;

        self.registrar_log(&format!(
            "[SWAP] Proceso P{} Página {} movida a Swap (Marco {} liberado)",
            pid_victima, pag_victima, idx_victima
        ));

        Ok(idx_victima)
    }
}
