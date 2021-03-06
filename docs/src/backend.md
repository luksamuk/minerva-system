# Back-End

<style>
svg:not(:root ) {
      max-width: 100%;
	  height: auto;
}
</style>

<center>

```dot process
graph {
	bgcolor=transparent;
	rankdir="TB";
	compound=true;
	node[style=filled; fillcolor="#999999"];
	
	frontend[label="FRONT-END", shape=note, color=darkorange, fontcolor=darkorange, fillcolor=transparent, width=2, height=1];
	rest[label="REST\n(API)", shape=box3d, color=green, fontcolor=green, width=1, height=0.75];
	
	subgraph cluster_db {
		rankdir="LR";
		label = "BANCO DE DADOS\n(multi-tenant)";
		color=darkmagenta;
		fontcolor=darkmagenta;
		db2[label="inq3", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
        db1[label="inq2", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
		db3[label="inq1", shape=cylinder, color=darkmagenta, fontcolor=darkmagenta];
		{rank=same; db3; db1; db2;}
	}

	subgraph cluster_backend {
		fontcolor=darkred;
		rankdir="LR";
		color=darkred;
		node[shape=hexagon; regular=true; fixedsize=true; width=1; height=1];
		
		runonce[label="RUNONCE", fillcolor="#DDDDDD", shape=doublecircle, color=darkblue, fontcolor=darkblue];
		report[label="REPORT", color=blue, fontcolor=blue];
		tenancy[label="TENANCY", color=blue, fontcolor=blue];
		user[label="USER", color=blue, fontcolor=blue];
		session[label="SESSION", color=blue, fontcolor=blue];
		product[label="PRODUCT", color=blue, fontcolor=blue];
		stock[label="STOCK", color=blue, fontcolor=blue];
		client[label="CLIENT", color=blue, fontcolor=blue];
		audit[label="AUDIT", color=blue, fontcolor=blue];
		comm[label="COMM", color=blue, fontcolor=blue];
			
		tenancy -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		user -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		session -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		product -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		stock -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		runonce -- db1 [lhead=cluster_db, color=magenta, fontcolor=darkmagenta, penwidth=2.0];
		client -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
		audit -- db1 [lhead=cluster_db, color=darkmagenta, fontcolor=darkmagenta, penwidth=2.0];
	}
	
	frontend -- rest[color=green, fontcolor=green, penwidth=2.0];
	rest -- tenancy[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- user[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- session[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- product[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- stock[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- client[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- audit[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- comm[color=blue, fontcolor=blue, penwidth=2.0];
	rest -- report[color=blue, fontcolor=blue, penwidth=2.0];
	
}
```

</center>

<div>

```dot process
graph {
	bgcolor=transparent;
	rankdir="TB";
	compound=true;
	node[style=filled; fillcolor=transparent];
	
	subgraph cluster_legenda {
		label = "Legenda";
		style = filled;
		fillcolor = white;
		key [label=<
		  <table border="0" cellpadding="2" cellspacing="0" cellborder="0">
			<tr><td align="right" port="i1">gRPC  </td></tr>
            <tr><td align="right" port="i2">BD  </td></tr>
            <tr><td align="right" port="i3">HTTP/REST  </td></tr>
          </table>
		>; shape=plaintext]
        key2 [label=<
		  <table border="0" cellpadding="2" cellspacing="0" cellborder="0">
            <tr><td port="i1">&nbsp;</td></tr>
            <tr><td port="i2">&nbsp;</td></tr>
            <tr><td port="i3">&nbsp;</td></tr>
          </table>
	    >; shape=plaintext]
		{rank=same; rankdir=LR; key; key2; }
		key:i1:e -- key2:i1:w [color=blue; penwidth=2.0];
		key:i2:e -- key2:i2:w [color=darkmagenta; penwidth=2.0];
		key:i3:e -- key2:i3:w [color=green; penwidth=2.0];
  }
}
```

</div>

O back-end Minerva comp??e-se de microsservi??os, com uma interface comum de
comunica????o externa que seja simples de usar para os padr??es atuais.

O back-end comp??e-se dos seguintes componentes:

1. **Componente de API:** um servi??o composto de rotas HTTP, sendo
   portanto uma API REST. Este servi??o requisita dados sob demanda a cada
   servi??o, dependendo do recurso que foi requisitado por via externa. ??
   efetivamente o intermedi??rio entre Minerva e o mundo externo. As
   requisi????es entre este servi??o e os outros dever??o ser feito atrav??s da
   abertura de uma requisi????o gRPC em que este m??dulo seja o cliente
   requisitante; as respostas recebidas via gRPC s??o ent??o retornadas
   como resposta ??s requisi????es recebidas via REST, ap??s tratamento para
   serializa????o como JSON.
2. **Componente de usu??rios:** Servidor gRPC respons??vel por realizar o CRUD
   de usu??rios e por verificar as regras de neg??cio destas opera????es.
3. **Componente de sess??o:** Servidor gRPC respons??vel por realizar login,
   logoff, verifica????o de senha e gerenciamento de sess??o de usu??rios.
4. **Componente de produtos:** Servidor gRPC respons??vel por realizar o CRUD
   de produtos e por verificar as regras de neg??cio destas opera????es.
5. **Componente de estoque:** Servidor gRPC respons??vel por realizar regras
   de neg??cios relacionadas a estoque de produtos (in??cio, baixa, lan??amento,
   etc).
6. **Componente de inquilinos:** Servidor gRPC respons??vel por coordenar a
   cria????o ou remo????o de novos inquilinos no sistema. Cada inquilino
   possuir?? seu pr??prio conjunto de dados, e isso afetar?? diretamente na
   infraestrutura reservada para o mesmo (cria????o ou remo????o de bancos
   de dados ou segmentos espec??ficos em certos servi??os).
7. **Componente de relat??rios:** Servidor gRPC respons??vel pela gera????o de
   relat??rios humanamente leg??veis, envolvendo portanto agrega????o de dados
   de acordo com o que for externamente requisitado.
8. **Componente de clientes:** Servidor gRPC respons??vel por realizar o CRUD
   e a coordena????o de dados de clientes do inquilino em quest??o.
9. **Componente de auditoria:** Servidor gRPC respons??vel por gerenciar a
   consulta ao logs de auditoria do sistema.
10. **Componente de comunica????o instant??nea:** Servidor gRPC para CRM atrav??s
	de comunica????o via canais de mensagens instant??neas.

Os **servi??os gRPC** supracitados tratam-se de servidores gRPC que podem
receber conex??es vindas do ponto de entrada REST ou mesmo entre si. Al??m
disso, os servi??os gRPC devem ser capazes de se comunicar com bancos de
dados, que s??o recursos essenciais para os mesmos (exemplo: PostgreSQL,
MongoDB, Redis). **Estes servi??os devem gravar log de suas opera????es**,
mais especificamente nas opera????es de inser????o, atualiza????o e exclus??o.

A API REST sempre se comunica diretamente com os servi??os gRPC, e os mesmos
s??o encorajados a se comunicarem entre si quando for necess??rio estabelecer
comunica????o bloqueante entre os mesmos. Todavia, quando for necess??rio
estabelecer comunica????o n??o-bloqueante entre os microsservi??os (leia-se,
quando o retorno para os usu??rios for desnecess??rio), ser?? feito o uso
de mensageria com despacho autom??tico, sem comunica????o gRPC direta.


## Bibliotecas

As bibliotecas planejadas para o sistema s??o:

- [x] `minerva-rpc`: Implementa????o de protocolos gRPC e de mensagens destes
   protocolo. Deve ser importado em todos os m??dulos, sendo essencial para
   a cria????o de clientes e servidores gRPC. Os modelos de comunica????o
   implementados para si devem ser tamb??m convertidos para e
  a partir dos DTOs do m??dulo de dados.
- [x] `minerva-data`: Implementa????o de utilit??rios de comunica????o com banco de
  dados (PostgreSQL) e objetos de transfer??ncia de dados (DTOs). Deve ser
  importado em todos os m??dulos, exceto na comunica????o REST. Os DTOs tamb??m
  devem implementar traits e utilit??rios para convers??o das mensagens
  implementadas em `minerva-rpc` para os DTOs desta biblioteca.
- [ ] `minerva-cache`: Implementa????o de utilit??rios de comunica????o com
  cache, message brokers e armazenamento tempor??rio _in-memory_ (Redis).
  Deve ser importado principalmente no m??dulo de sess??o.

## M??dulos

Os m??dulos planejados para o sistema s??o:

- [ ] `minerva-tenancy`: Servidor gRPC para CRUD de inquilinos. Deve ser
  capaz de gerenciar inquilinos, mas um inquilino n??o pode ser deletado
  atrav??s desse servi??o, apenas desativado. Apenas administradores do sistema
  podem ter acesso.
- [x] `minerva-user`: Servidor gRPC para CRUD de usu??rios. Deve ser capaz de
  manipular as regras de neg??cios relacionadas a clientes.
- [x] `minerva-session`: Servidor gRPC para ger??ncia de sess??o de usu??rio.
- [ ] `minerva-product`: Servidor gRPC para CRUD de produtos. Deve ser capaz
  de manipular as regras de neg??cios relacionadas a produtos, mas que n??o
  envolvam controle de estoque.
- [ ] `minerva-stock`: Servidor gRPC para CRUD de estoque de produtos. Deve
  ser capaz de manipular as regras de neg??cios relacionadas a estoque, mas
  que n??o envolvam manipula????o de produtos.
- [x] `minerva-rest`: Servidor REST para comunica????o com os demais m??dulos
  execut??veis. Possui rotas que apontam para servi??os espec??ficos, e ?? por
  defini????o um cliente gRPC de todos os servidores gRPC.
- [x] `minerva-runonce`: Servi??o **avulso** para configura????o do ambiente, de
  forma ass??ncrona. Respons??vel pela execu????o de migra????es do banco de dados
  e outras opera????es de configura????o inicial.
- [ ] `minerva-report`: Servidor gRPC para gera????o de relat??rios. Deve receber
  dados com formata????o esperada de um relat??rio, e ent??o dever?? gerar um
  arquivo PDF e retorn??-lo inteiramente como resposta.
- [ ] `minerva-client`: Servidor gRPC para CRUD de clientes. Deve ser capaz
  de manipular as regras de neg??cios relacionadas a clientes.
- [ ] `minerva-audit`: Servidor gRPC para gerenciamento de logs de auditoria.
  Possibilita a consulta aos logs de auditoria do sistema.
- [ ] `minerva-comm`: Servidor gRPC para comunica????o externa com clientes
  via mensagens instant??neas.

## Portas

Os servi??os, independente de serem gRPC ou REST, devem ser executados em
certas portas padr??o para evitarem conflitos durante o tempo de depura????o.
Cada porta deve tamb??m ser configur??vel atrav??s de vari??veis de ambiente.

A tabela a seguir discrimina as vari??veis de ambiente e as portas padr??o
de acordo com o servi??o em quest??o.

| Servi??o | Vari??vel               | Valor |
|---------|------------------------|-------|
| REST    | `ROCKET_PORT`          | 9000  |
| USER    | `USER_SERVICE_PORT`    | 9010  |
| SESSION | `SESSION_SERVICE_PORT` | 9011  |

<!-- | PRODUCT | `PRODUCT_SERVICE_PORT` | 9012  | -->
<!-- | STOCK   | `STOCK_SERVICE_PORT`   | 9013  | -->
<!-- | REPORT  | `REPORT_SERVICE_PORT`  | 9014  | -->
<!-- | TENANCY | `TENANCY_SERVICE_PORT` | 9015  | -->
<!-- | CLIENT  | `CLIENT_SERVICE_PORT`  | 9016  | -->
<!-- | AUDIT   | `AUDIT_SERVICE_PORT`   | 9017  | -->
<!-- | COMM    | `COMM_SERVICE_PORT`    | 9018  | -->


No caso do servi??o REST, verifique o arquivo `Rocket.toml` para avaliar
a configura????o em desenvolvimento e em produ????o do mesmo.

## Gateways

Os servi??os tamb??m podem operar em m??quinas diferentes, dependendo de sua
rota.

Normalmente, quando todos os servi??os s??o executados manualmente na mesma
m??quina, operamos com uma rota `localhost`. Nesse caso, a vari??vel de
ambiente de cada servi??o ?? definida como esse valor.

Todavia, num ambiente de orquestra????o de cont??ineres (como Docker Compose
ou Kubernetes), cada servi??o estar?? operando de forma separada, e poder??
comunicar-se com os outros servi??os por interm??dio de uma rede interna
ao qual apenas os servi??os t??m acesso de forma expl??cita. Assim, as
vari??veis de ambiente que determinam o nome do servidor devem ser definidas
manualmente, de acordo com a forma como o deploy de cada servi??o foi
realizado.

A seguir, temos uma tabela relacionando os sistemas com sas vari??veis de
ambiente. Os valores das vari??veis ser??o definidos de acordo com o orquestrador
de cont??ineres sendo utilizado.

No caso do servi??o REST, verifique o arquivo `Rocket.toml` para avaliar
a configura????o em desenvolvimento e em produ????o do mesmo.


| Servi??o              | Vari??vel de ambiente      |
|----------------------|---------------------------|
| Banco de dados SQL   | `DATABASE_SERVICE_SERVER` |
| Banco de dados NoSQL | `MONGO_SERVICE_SERVER`    |
| Cache Redis          | `REDIS_SERVICE_SERVER`    |
|----------------------|---------------------------|
| REST                 | `REST_SERVICE_SERVER`     |
| USER                 | `USER_SERVICE_SERVER`     |
| SESSION              | `SESSION_SERVICE_SERVER`  |

