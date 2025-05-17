```mermaid
graph TD
    subgraph Application
        my_http_server_app
    end

    subgraph Core HTTP Components
        http_router
        http_server_core
        http_service
    end

    subgraph HTTP Protocol Abstractions
        http_types
        http_parser
        http_serializer
    end

    subgraph Foundation & Utilities
        tokio_net_utils
        common_utils
    end

    my_http_server_app --> http_router
    my_http_server_app --> http_server_core
    my_http_server_app --> tokio_net_utils
    my_http_server_app --> common_utils

    http_router --> http_service
    http_server_core --> http_service
    http_server_core --> http_parser
    http_server_core --> http_serializer
    http_server_core --> tokio_net_utils
    http_server_core --> common_utils

    http_parser --> http_types
    http_serializer --> http_types

    http_service --> http_types
    http_types --> common_utils
    tokio_net_utils --> common_utils
```
