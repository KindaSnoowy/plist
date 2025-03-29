# PList - TODO List para CLI em Rust

**PList** é um programa de linha de comando (CLI) para controle de afazeres. Utiliza JSON para armazenar os dados de maneira persistente.

## Funcionalidades

- **Criar nova tarefa**: Adiciona uma nova tarefa à lista.
- **Listar tarefas**: Exibe todas as tarefas registradas.
- **Completar tarefa**: Marca uma tarefa como completada.
- **Desmarcar tarefa**: Marca uma tarefa como não completada.
- **Excluir tarefa**: Exclui uma tarefa da lista.
- **Ajuda**: Exibe informações sobre os comandos disponíveis.

## Como usar

### Comandos disponíveis:

- `new <task_name>`: Cria uma nova tarefa com o nome fornecido.
- `list`: Lista todas as tarefas.
- `complete <task_id>`: Marca a tarefa com o ID especificado como concluída.
- `uncomplete <task_id>`: Marca a tarefa com o ID especificado como não concluída.
- `delete <task_id>`: Exclui a tarefa com o ID especificado.
- `help`: Exibe informações sobre os comandos disponíveis.

### Exemplo de uso:

#### Criar uma nova tarefa:
```bash
$ ./plist new <task_name>
```

#### Listar tarefas:
```bash
$ ./plist list
Task ID: 1, Name: Tarefa 1, Completed: ☑
Task ID: 2, Name: Tarefa 2, Completed: ☒
```

#### Completar/Descompletar(?) tarefas:
```bash
$ ./plist complete/uncomplete <task_id>
Task <task_name> marked as completed/uncompleted!
./plist list
Task ID: 1, Name: Tarefa 1, Completed: ☒
Task ID: 2, Name: Tarefa 2, Completed: ☑
```

#### Completar/Descompletar(?) tarefas:
```bash
$ ./plist complete/uncomplete <task_id>
Task <task_name> marked as completed/uncompleted!
./plist list
Task ID: 1, Name: Tarefa 1, Completed: ☒
Task ID: 2, Name: Tarefa 2, Completed: ☑
```


#### Deletar tarefas:
```bash
$ ./plist delete <task_id>
Task with ID {task_id} deleted
```

### Dependências:
- `serde` para serialização e desserialização de dados JSON.
- `serde_json` para manipulação dos dados em formato JSON.
